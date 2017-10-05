
use std::io::Read;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use reqwest::{Url, StatusCode, Body};
use reqwest::Client as ReqwestClient;
use reqwest::header::{Headers, Authorization, Bearer, ContentType};
use reqwest::Response as ReqwestResponse;

use super::Dropbox;
use errors::*;

pub mod header;

use self::header::DropboxAPIArg;

#[derive(Debug)]
pub struct Response<T: DeserializeOwned> {
    pub body: T,
    pub status: StatusCode,
    pub headers: Headers,
}

// TODO when TryFrom is stabilized
//impl<T> TryFrom<ReqwestResponse> for Response<T>
impl<T> Response<T>
    where T: DeserializeOwned
{
    pub fn try_from(resp: ReqwestResponse) -> Result<Response<T>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let body = serde_json::from_reader(resp)?;
            Ok(Response {
                body: body,
                status: status,
                headers: headers,
            })
        } else {
            bail!(build_error(resp)?)
        }
    }
}

pub trait Client {
    fn access_token(&self) -> &str;
    fn rpc_request<T, R>(&self, url: Url, request_body: T) -> Result<Response<R>>
        where T: Serialize,
              R: DeserializeOwned;
    fn content_upload_request<T, S, R>(&self,
                                       url: Url,
                                       request_body: T,
                                       contents: S)
                                       -> Result<Response<R>>
        where T: DeserializeOwned + Serialize + Sync + Clone + Send + 'static,
              S: Into<Body>,
              R: DeserializeOwned;
}

impl Client for Dropbox {
    fn access_token(&self) -> &str {
        self.access_token.as_ref()
    }

    fn rpc_request<T, R>(&self, url: Url, request_body: T) -> Result<Response<R>>
        where T: Serialize,
              R: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .json(&request_body)
            .send()?;

        Ok(Response::try_from(res)?)
    }

    fn content_upload_request<T, S, R>(&self,
                                       url: Url,
                                       request_body: T,
                                       contents: S)
                                       -> Result<Response<R>>
        where T: DeserializeOwned + Serialize + Sync + Clone + Send + 'static,
              S: Into<Body>,
              R: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(request_body))
            .body(contents)
            .send()?;

        Ok(Response::try_from(res)?)
    }
}
