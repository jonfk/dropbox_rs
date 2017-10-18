
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
pub enum ResponseWithErr<T> {
    Ok {
        body: T,
        status: StatusCode,
        headers: Headers,
    },
    Err(String),
}

// TODO when TryFrom is stabilized
//impl<T> TryFrom<ReqwestResponse> for Response<T>
impl<T> ResponseWithErr<T>
    where T: DeserializeOwned
{
    pub fn try_from(mut resp: ReqwestResponse) -> Result<ResponseWithErr<T>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let mut body = String::new();
            resp.by_ref().read_to_string(&mut body)?;
            // TODO replace with proper log.debug
            println!("\nRPC json: {}\n", body);
            //let body = serde_json::from_reader(resp)?;
            let json = serde_json::from_str(body.as_str())?;
            Ok(ResponseWithErr::Ok {
                body: json,
                status: status,
                headers: headers,
            })
        } else {
            let mut body = String::new();
            resp.by_ref().read_to_string(&mut body)?;
            Ok(ResponseWithErr::Err(body))
        }
    }
}

pub enum ContentResponseWithErr<T, C: Read> {
    Ok {
        body: T,
        content: C,
        status: StatusCode,
        headers: Headers,
    },
    Err(String),
}

impl<T> ContentResponseWithErr<T, ReqwestResponse>
    where T: DeserializeOwned
{
    pub fn try_from(mut resp: ReqwestResponse)
                    -> Result<ContentResponseWithErr<T, ReqwestResponse>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        if status.is_success() {
            let raw_header = headers.get_raw(DROPBOX_API_RESULT)
                .ok_or_else(|| HeaderNotFound(DROPBOX_API_RESULT.to_owned()))?;
            let raw_header_contents: Vec<u8> =
                raw_header.into_iter().flat_map(|l| l.to_vec()).collect::<_>();
            let body = serde_json::from_slice(&raw_header_contents)?;
            Ok(ContentResponseWithErr::Ok {
                body: body,
                content: resp,
                status: status,
                headers: headers.clone(),
            })
        } else {
            let mut body = String::new();
            resp.by_ref().read_to_string(&mut body)?;
            Ok(ContentResponseWithErr::Err(body))
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
    fn rpc_request<T, R>(&self, url: Url, request_body: T) -> Result<ResponseWithErr<R>>
        where T: Serialize,
              R: DeserializeOwned;
}

impl<C> RPCClient for C
    where C: HasAccessToken + Clone
{
    fn rpc_request<T, R>(&self, url: Url, request_body: T) -> Result<ResponseWithErr<R>>
        where T: Serialize,
              R: DeserializeOwned
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
    fn content_upload_request<T, S, R>(&self,
                                       url: Url,
                                       request_body: T,
                                       contents: S)
                                       -> Result<ResponseWithErr<R>>
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
                                       -> Result<ResponseWithErr<R>>
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

        Ok(ResponseWithErr::try_from(res)?)
    }
}

pub trait ContentDownloadClient<C: Read> {
    fn content_download<T, R>(&self, url: Url, request: T) -> Result<ContentResponseWithErr<R, C>>
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
                              -> Result<ContentResponseWithErr<R, ReqwestResponse>>
        where T: Serialize,
              R: DeserializeOwned
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
