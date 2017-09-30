
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

pub struct Dropbox {
    access_token: String,
}
