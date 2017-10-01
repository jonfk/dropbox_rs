
use serde_urlencoded;
use serde_json;
use reqwest::{Url, Client};
use std::collections::HashMap;
use std::io::{self, Write};

use dropbox::auth::AuthorizationResponse::{CodeResponse, TokenResponse};

use dropbox::errors::*;

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
        team_id: Option<String>,
        state: Option<String>,
    },
    CodeResponse { code: String, state: Option<String> },
}

pub fn parse_authorization_response(redirect_uri: &str) -> Option<AuthorizationResponse> {
    let mut url = Url::parse(redirect_uri).unwrap();
    if url.query().is_none() {
        let fragment = url.fragment().map(|x| String::from(x));
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
            uid: query_pairs.remove("uid").unwrap_or("".to_owned()),
            account_id: query_pairs.remove("account_id").unwrap_or("".to_owned()),
            team_id: query_pairs.remove("team_id"),
            state: query_pairs.remove("state"),
        })
    } else {
        None
    }
}

pub fn run_authorization_flow(client_id: &str,
                              redirect_uri: &str,
                              response_type: &str)
                              -> Option<AuthorizationResponse> {
    println!("Please visit the following url and authorize this app {}",
             build_authorization_uri(client_id, redirect_uri, response_type));
    print!("Paste  redirect url here : ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    parse_authorization_response(&buffer)
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

        let client = Client::new()?;
        let mut res = client.post(url)?
            .send()?;
        let mut buf = Vec::with_capacity(10000);
        io::copy(&mut res, &mut buf)?;

        println!("{:?}", res);
        println!("{:?}", String::from_utf8(buf.clone())?);

        Ok(serde_json::from_slice(&buf)?)
    }
}
