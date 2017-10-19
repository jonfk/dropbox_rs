
use std::io::Read;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use reqwest::{Url, StatusCode, Body};
use reqwest::Client as ReqwestClient;
use reqwest::header::{Headers, Authorization, Bearer, ContentType};
use reqwest::Response as ReqwestResponse;

use super::Dropbox;
use self::errors::*;
use self::errors::ErrorKind::HeaderNotFound;

pub mod errors;
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

#[derive(Debug)]
pub struct ContentResponse<T, C: Read> {
    pub body: T,
    pub content: C,
    pub status: StatusCode,
    pub headers: Headers,
}

#[derive(Debug)]
pub enum ResponseWithErr<T, E> {
    Ok(Response<T>),
    Err(APIError<E>),
}

// TODO when TryFrom is stabilized
//impl<T> TryFrom<ReqwestResponse> for Response<T>
impl<T, E> ResponseWithErr<T, E>
    where T: DeserializeOwned,
          E: DeserializeOwned
{
    pub fn try_from(mut resp: ReqwestResponse) -> Result<ResponseWithErr<T, E>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let mut body = String::new();
            resp.by_ref().read_to_string(&mut body)?;
            // TODO replace with proper log.debug
            println!("\nRPC json: {}\n", body);
            //let body = serde_json::from_reader(resp)?;
            let json = serde_json::from_str(body.as_str())?;
            Ok(ResponseWithErr::Ok(Response {
                body: json,
                status: status,
                headers: headers,
            }))
        } else {
            let mut error_body = String::new();
            resp.by_ref().read_to_string(&mut error_body)?;
            let json: DropboxError<E> = serde_json::from_str(error_body.as_str())?;
            Ok(ResponseWithErr::Err(APIError {
                body: error_body,
                status: status,
                error: json.error,
                user_message: json.user_message,
            }))
        }
    }
}

#[derive(Debug)]
pub enum ContentResponseWithErr<T, C: Read, E> {
    Ok(ContentResponse<T, C>),
    Err(APIError<E>),
}

impl<T, E> ContentResponseWithErr<T, ReqwestResponse, E>
    where T: DeserializeOwned,
          E: DeserializeOwned
{
    pub fn try_from(mut resp: ReqwestResponse)
                    -> Result<ContentResponseWithErr<T, ReqwestResponse, E>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let raw_header = headers.get_raw(DROPBOX_API_RESULT)
                .ok_or_else(|| HeaderNotFound(DROPBOX_API_RESULT.to_owned()))?;
            let raw_header_contents: Vec<u8> =
                raw_header.into_iter().flat_map(|l| l.to_vec()).collect::<_>();
            let body = serde_json::from_slice(&raw_header_contents)?;
            Ok(ContentResponseWithErr::Ok(ContentResponse {
                body: body,
                content: resp,
                status: status,
                headers: headers.clone(),
            }))
        } else {
            let mut error_body = String::new();
            resp.by_ref().read_to_string(&mut error_body)?;
            let json: DropboxError<E> = serde_json::from_str(error_body.as_str())?;
            Ok(ContentResponseWithErr::Err(APIError {
                body: error_body,
                status: status,
                error: json.error,
                user_message: json.user_message,
            }))
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
    fn rpc_request<T, R, E>(&self, url: Url, request_body: T) -> Result<ResponseWithErr<R, E>>
        where T: Serialize,
              R: DeserializeOwned,
              E: DeserializeOwned;
}

impl<C> RPCClient for C
    where C: HasAccessToken + Clone
{
    fn rpc_request<T, R, E>(&self, url: Url, request_body: T) -> Result<ResponseWithErr<R, E>>
        where T: Serialize,
              R: DeserializeOwned,
              E: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .json(&request_body)
            .send()?;

        Ok(ResponseWithErr::try_from(res)?)
    }
}

pub trait ContentUploadClient {
    fn content_upload_request<T, S, R, E>(&self,
                                          url: Url,
                                          request_body: T,
                                          contents: S)
                                          -> Result<ResponseWithErr<R, E>>
        where T: Serialize,
              S: Into<Body>,
              R: DeserializeOwned,
              E: DeserializeOwned;
}

impl<C> ContentUploadClient for C
    where C: HasAccessToken + Clone
{
    fn content_upload_request<T, S, R, E>(&self,
                                          url: Url,
                                          request_body: T,
                                          contents: S)
                                          -> Result<ResponseWithErr<R, E>>
        where T: Serialize,
              S: Into<Body>,
              R: DeserializeOwned,
              E: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(serde_json::to_string(&request_body)?))
            .body(contents)
            .send()?;

        Ok(ResponseWithErr::try_from(res)?)
    }
}

pub trait ContentDownloadClient<C: Read> {
    fn content_download<T, R, E>(&self,
                                 url: Url,
                                 request: T)
                                 -> Result<ContentResponseWithErr<R, C, E>>
        where T: Serialize,
              R: DeserializeOwned,
              C: Read,
              E: DeserializeOwned;
}


impl<C> ContentDownloadClient<ReqwestResponse> for C
    where C: HasAccessToken + Clone
{
    fn content_download<T, R, E>(&self,
                                 url: Url,
                                 request: T)
                                 -> Result<ContentResponseWithErr<R, ReqwestResponse, E>>
        where T: Serialize,
              R: DeserializeOwned,
              E: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(serde_json::to_string(&request)?))
            .send()?;

        Ok(ContentResponseWithErr::try_from(res)?)
    }
}
