
const BUFFER_SIZE: usize = 100000;

pub mod auth;
pub mod paper;
pub mod response;
pub mod errors {
    use super::BUFFER_SIZE;

    pub use reqwest::StatusCode;
    use reqwest::Response;

    use std::io;
    use std::string::String;

    #[derive(Debug)]
    pub struct APIError {
        status: StatusCode,
        body: String,
    }

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
            API(api_error: APIError) {
                description("An error occurred when interacting with Dropbox"),
                display("{:?}", api_error),
            }
        }
    }

    impl From<APIError> for ErrorKind {
        fn from(api_error: APIError) -> Self {
            ErrorKind::API(api_error)
        }
    }

    pub fn build_error(mut resp: Response) -> Result<ErrorKind> {
        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        io::copy(&mut resp, &mut buf)?;

        Ok(APIError {
                status: resp.status(),
                body: String::from_utf8(buf)?,
            }
            .into())
    }
}

use dropbox::paper::PaperOperations;

pub struct Dropbox {
    access_token: String,
    pub paper_ops: PaperOperations,
}

impl Dropbox {
    pub fn new(access_token: &str) -> Dropbox {
        Dropbox {
            access_token: String::from(access_token),
            paper_ops: PaperOperations::new(access_token),
        }
    }
}
