
pub mod auth;
pub mod paper;
pub mod errors {
    error_chain!{
        foreign_links {
            Url(::reqwest::UrlError);
            Reqwest(::reqwest::Error);
            Utf8(::std::string::FromUtf8Error);
            Io(::std::io::Error);
            Json(::serde_json::Error);
            UrlEncodedSer(::serde_urlencoded::ser::Error);
        }
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
