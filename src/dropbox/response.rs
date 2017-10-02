
use serde::de::DeserializeOwned;
use serde_json;
use reqwest::StatusCode;
use reqwest::header::Headers;
use reqwest::Response as ReqwestResponse;

use std::io;

use dropbox::errors::*;

const BUFFER_SIZE: usize = 100000;

#[derive(Debug)]
pub struct Response<T: DeserializeOwned> {
    body: T,
    status: StatusCode,
    headers: Headers,
}

// TODO when TryFrom is stabilized
//impl<T> TryFrom<ReqwestResponse> for Response<T>
impl<T> Response<T>
    where T: DeserializeOwned
{
    pub fn try_from(resp: ReqwestResponse) -> Result<Response<T>> {
        let status = resp.status();
        let headers = resp.headers().clone();

        let body = serde_json::from_reader(resp)?;
        Ok(Response {
            body: body,
            status: status,
            headers: headers,
        })
    }
}
