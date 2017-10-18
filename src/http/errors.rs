
pub use reqwest::StatusCode;
use reqwest::Response;

use serde_json;
use serde::de::DeserializeOwned;

use std::string::String;
use std::io::Read;


error_chain!{
    foreign_links {
        Url(::reqwest::UrlError);
        Reqwest(::reqwest::Error);
        Utf8(::std::string::FromUtf8Error);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        UrlEncodedSer(::serde_urlencoded::ser::Error);
    }
    errors {
        HeaderNotFound(header: String) {
            description("An expected header wasn't found"),
            display("Couldn't find header: {}", header),
        }
    }
}

#[derive(Debug)]
pub struct APIError<E> {
    pub status: StatusCode,
    pub body: String,
    pub error: E,
    pub user_message: Option<String>,
}

impl<E> APIError<E>
    where E: DeserializeOwned
{
    pub fn build_error(status: StatusCode, error_body: String) -> Result<APIError<E>> {
        let json: DropboxError<E> = serde_json::from_str(error_body.as_str())?;

        Ok(APIError {
            status: status,
            body: error_body,
            error: json.error,
            user_message: json.user_message,
        })
    }
}

#[derive(Deserialize)]
pub struct DropboxError<T> {
    pub error_summary: String,
    pub error: T,
    pub user_message: Option<String>,
}
