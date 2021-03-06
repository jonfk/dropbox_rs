
use std::io::{self, Read};
use std::fmt;
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

pub struct ContentResponse<T> {
    pub body: T,
    content: Box<Read>,
    pub status: StatusCode,
    pub headers: Headers,
}

impl<T> fmt::Debug for ContentResponse<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ContentResponse {{ body: {:?}, status: {:?}, headers: {:?} }}",
               self.body,
               self.status,
               self.headers)
    }
}

impl<T> Read for ContentResponse<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.content.read(buf)
    }
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
            info!("[response_body = {}]", body);
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
pub enum ContentResponseWithErr<T, E> {
    Ok(ContentResponse<T>),
    Err(APIError<E>),
}

impl<T, E> ContentResponseWithErr<T, E>
    where T: DeserializeOwned,
          E: DeserializeOwned
{
    pub fn try_from(mut resp: ReqwestResponse) -> Result<ContentResponseWithErr<T, E>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let raw_header = headers.get_raw(DROPBOX_API_RESULT)
                .ok_or_else(|| HeaderNotFound(DROPBOX_API_RESULT.to_owned()))?;
            let raw_header_contents: Vec<u8> =
                raw_header.into_iter().flat_map(|l| l.to_vec()).collect::<_>();
            let body = serde_json::from_slice(&raw_header_contents)?;
            //info!("[response_body = {}]", body);
            Ok(ContentResponseWithErr::Ok(ContentResponse {
                body: body,
                content: Box::new(resp),
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
        let req_arg = serde_json::to_string(&request_body)?;
        info!("[RPC] [url = {}] [request_body = {}]", url, req_arg);
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::json())
            .body(req_arg)
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
        let req_arg = serde_json::to_string(&request_body)?;
        info!("[ContentUpload] [url = {}] [request_body = {}]",
              url,
              req_arg);
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(req_arg))
            .body(contents)
            .send()?;

        Ok(ResponseWithErr::try_from(res)?)
    }
}

pub trait ContentDownloadClient<C: Read> {
    fn content_download<T, R, E>(&self,
                                 url: Url,
                                 request: T)
                                 -> Result<ContentResponseWithErr<R, E>>
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
                                 -> Result<ContentResponseWithErr<R, E>>
        where T: Serialize,
              R: DeserializeOwned,
              E: DeserializeOwned
    {
        let client = ReqwestClient::new();
        let req_arg = serde_json::to_string(&request)?;
        info!("[ContentUpload] [url = {}] [request_body = {}]",
              url,
              req_arg);
        let res = client.post(url)
            .header(Authorization(Bearer { token: self.access_token().to_owned() }))
            .header(ContentType::octet_stream())
            .header(DropboxAPIArg(req_arg))
            .send()?;

        Ok(ContentResponseWithErr::try_from(res)?)
    }
}
