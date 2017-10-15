
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
use errors::ErrorKind::HeaderNotFound;

pub mod header {
    header! { (DropboxAPIArg, "Dropbox-API-Arg") => [String] }
}

use self::header::DropboxAPIArg;

static DROPBOX_API_RESULT: &'static str = "Dropbox-API-Result";

#[derive(Debug)]
pub struct Response<T> {
    pub body: T,
    pub status: StatusCode,
    pub headers: Headers,
}

// TODO when TryFrom is stabilized
//impl<T> TryFrom<ReqwestResponse> for Response<T>
impl<T> Response<T>
    where T: DeserializeOwned
{
    pub fn try_from(mut resp: ReqwestResponse) -> Result<Response<T>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let mut body = String::new();
            resp.by_ref().read_to_string(&mut body)?;
            // TODO replace with proper log.debug
            println!("\nRPC json: {}\n", body);
            //let body = serde_json::from_reader(resp)?;
            let json = serde_json::from_str(body.as_str())?;
            Ok(Response {
                body: json,
                status: status,
                headers: headers,
            })
        } else {
            bail!(build_error(resp)?)
        }
    }
}

pub struct ContentResponse<T, C: Read> {
    pub body: T,
    pub content: C,
    pub status: StatusCode,
    pub headers: Headers,
}

impl<T> ContentResponse<T, ReqwestResponse>
    where T: DeserializeOwned
{
    pub fn try_from(resp: ReqwestResponse) -> Result<ContentResponse<T, ReqwestResponse>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let raw_header = headers.get_raw(DROPBOX_API_RESULT)
                .ok_or_else(|| HeaderNotFound(DROPBOX_API_RESULT.to_owned()))?;
            let raw_header_contents: Vec<u8> =
                raw_header.into_iter().flat_map(|l| l.to_vec()).collect::<_>();
            let body = serde_json::from_slice(&raw_header_contents)?;
            Ok(ContentResponse {
                body: body,
                content: resp,
                status: status,
                headers: headers.clone(),
            })
        } else {
            bail!(build_error(resp)?)
        }
    }
}

pub trait HasAccessToken {
    fn access_token(&self) -> &str;
}

impl HasAccessToken for Dropbox {
    fn access_token(&self) -> &str {
        self.access_token.as_ref()
    }
}

pub trait RPCClient {
    fn rpc_request<T, R>(&self, url: Url, request_body: T) -> Result<Response<R>>
        where T: Serialize,
              R: DeserializeOwned;
}

impl<C> RPCClient for C
    where C: HasAccessToken + Clone
{
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
}

pub trait ContentUploadClient {
    fn content_upload_request<T, S, R>(&self,
                                       url: Url,
                                       request_body: T,
                                       contents: S)
                                       -> Result<Response<R>>
        where T: Serialize,
              S: Into<Body>,
              R: DeserializeOwned;
}

impl<C> ContentUploadClient for C
    where C: HasAccessToken + Clone
{
    fn content_upload_request<T, S, R>(&self,
                                       url: Url,
                                       request_body: T,
                                       contents: S)
                                       -> Result<Response<R>>
        where T: Serialize,
              S: Into<Body>,
              R: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(serde_json::to_string(&request_body)?))
            .body(contents)
            .send()?;

        Ok(Response::try_from(res)?)
    }
}

pub trait ContentDownloadClient<C: Read> {
    fn content_download<T, R>(&self, url: Url, request: T) -> Result<ContentResponse<R, C>>
        where T: Serialize,
              R: DeserializeOwned,
              C: Read;
}


impl<C> ContentDownloadClient<ReqwestResponse> for C
    where C: HasAccessToken + Clone
{
    fn content_download<T, R>(&self,
                              url: Url,
                              request: T)
                              -> Result<ContentResponse<R, ReqwestResponse>>
        where T: Serialize,
              R: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(serde_json::to_string(&request)?))
            .send()?;

        Ok(ContentResponse::try_from(res)?)
    }
}
