
use serde_urlencoded;
use serde_json;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use reqwest::{Url, Client};
use std::collections::HashMap;

use auth::AuthorizationResponse::{CodeResponse, TokenResponse};

use self::errors::*;
use http::{Response, ResponseWithErr, RPCClient};

static BASE_URL: &'static str = "https://api.dropboxapi.com/2/auth/token/";

pub fn build_authorization_uri(client_id: &str, redirect_uri: &str, response_type: &str) -> String {
    let mut url = Url::parse("https://www.dropbox.com/oauth2/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", response_type);
    url.as_str().to_owned()
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthorizationResponse {
    TokenResponse {
        access_token: String,
        token_type: String,
        uid: String,
        account_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        team_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    CodeResponse { code: String, state: Option<String> },
}

pub fn parse_authorization_response(redirect_uri: &str) -> Option<AuthorizationResponse> {
    match Url::parse(redirect_uri) {
        Err(_) => None,
        Ok(mut url) => {
            if url.query().is_none() {
                let fragment = url.fragment().map(String::from);
                url.set_query(fragment.as_ref().map(|x| x.as_str()));
            }

            let mut query_pairs = url.query_pairs().fold(HashMap::new(), |mut pairs, query_pair| {
                let (a, b) = query_pair;
                pairs.insert(a.into_owned(), b.into_owned());
                pairs
            });

            // TODO deserialize with serde_urlencoded
            if query_pairs.contains_key("code") {
                Some(CodeResponse {
                    code: query_pairs.remove("code").unwrap(),
                    state: query_pairs.remove("state"),
                })
            } else if query_pairs.contains_key("access_token") {
                Some(TokenResponse {
                    access_token: query_pairs.remove("access_token").unwrap(),
                    token_type: query_pairs.remove("token_type").unwrap(),
                    uid: query_pairs.remove("uid").unwrap_or_else(|| "".to_owned()),
                    account_id: query_pairs.remove("account_id").unwrap_or_else(|| "".to_owned()),
                    team_id: query_pairs.remove("team_id"),
                    state: query_pairs.remove("state"),
                })
            } else {
                None
            }
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct AuthTokenRequest {
    code: String,
    grant_type: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

#[derive(Debug)]
pub struct AuthOperations {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    _secret: (),
}

impl AuthOperations {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> AuthOperations {
        AuthOperations {
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
            redirect_uri: String::from(redirect_uri),
            _secret: (),
        }
    }
    pub fn fetch_token(&self, code: &str) -> Result<AuthorizationResponse> {
        let token_req = AuthTokenRequest {
            code: String::from(code),
            grant_type: String::from("authorization_code"),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            redirect_uri: self.redirect_uri.clone(),
        };
        let mut url = Url::parse("https://api.dropboxapi.com/oauth2/token")?;
        url.set_query(Some(serde_urlencoded::to_string(token_req)?.as_str()));

        let client = Client::new();
        let res = client.post(url)
            .send()?;

        println!("{:?}", res);
        Ok(serde_json::from_reader(res)?)
    }

    fn rpc_request<T, R, E>(&self, url: Url, request_body: T) -> Result<ResponseWithErr<R, E>>
        where T: Serialize,
              R: DeserializeOwned,
              E: DeserializeOwned
    {
        let client = Client::new();
        let res = client.post(url)
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.as_str()))
            .json(&request_body)
            .send()?;

        Ok(ResponseWithErr::try_from(res)?)
    }
    // TODO fix error
    pub fn token_from_oauth1(&self) -> Result<Response<TokenFromOAuth1Result>> {
        let url = Url::parse("https://api.dropboxapi.com/2/auth/token/from_oauth1")?;
        let resp_with_err: ResponseWithErr<_, TokenFromOAuth1Error> = self.rpc_request(url,
                         &TokenFromOAuth1Arg {
                             oauth1_token: self.client_id.clone(),
                             oauth1_token_secret: self.client_secret.clone(),
                         })?;
        match resp_with_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::TokenFromOAuth1Err(e).into()),
        }
    }
}

pub trait RevokableToken {
    fn revoke_token(&self) -> Result<Response<()>>;
}

impl<C> RevokableToken for C
    where C: RPCClient
{
    fn revoke_token(&self) -> Result<Response<()>> {
        let url = Url::parse(BASE_URL)?.join("revoke")?;
        let resp_with_err: ResponseWithErr<_, ()> = self.rpc_request(url, ())?;

        match resp_with_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(_) => unreachable!(),
        }
    }
}

#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct TokenFromOAuth1Arg {
    pub oauth1_token: String,
    pub oauth1_token_secret: String,
}

#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct TokenFromOAuth1Result {
    pub oauth2_token: String,
}

mod errors {
    use http::errors::APIError;

    error_chain!{
        links {
            Http(::http::errors::Error, ::http::errors::ErrorKind);
        }
        foreign_links {
            Url(::reqwest::UrlError);
            Reqwest(::reqwest::Error);
            Utf8(::std::string::FromUtf8Error);
            Io(::std::io::Error);
            Json(::serde_json::Error);
            UrlEncodedSer(::serde_urlencoded::ser::Error);
        }
        errors {
            TokenFromOAuth1Err(error: APIError<TokenFromOAuth1Error>) {
                description("TokenFromOAuth1Error"),
                display("{:?}", error)
            }
        }
    }

    #[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
    #[serde(tag = ".tag", rename_all = "snake_case")]
    pub enum TokenFromOAuth1Error {
        InvalidOauth1TokenInfo,
        AppIdMismatch,
    }

    impl From<APIError<TokenFromOAuth1Error>> for ErrorKind {
        fn from(error: APIError<TokenFromOAuth1Error>) -> Self {
            ErrorKind::TokenFromOAuth1Err(error)
        }
    }
}
