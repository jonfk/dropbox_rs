
use http::errors::APIError;

error_chain!{
    links {
        Http(::http::errors::Error, ::http::errors::ErrorKind);
    }
    foreign_links {
        Url(::reqwest::UrlError);
        Reqwest(::reqwest::Error);
        Utf8(::std::string::FromUtf8Error);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        UrlEncodedSer(::serde_urlencoded::ser::Error);
    }
    errors {
        DocLookupErr(error: APIError<DocLookupError>) {
            description("DocLookupError"),
            display("{:?}", error)
        }
        PaperDocCreateErr(error: APIError<PaperDocCreateError>) {
            description("PaperDocCreateError"),
            display("{:?}",error)
        }
        ListUsersCursorErr(error: APIError<ListUsersCursorError>) {
            description("ListUsersCursorError"),
            display("{:?}", error),
        }
        PaperApiCursorErr(error: APIError<PaperApiCursorError>) {
            description("PaperApiCursorError"),
            display("{:?}", error)
        }
        ListDocsCursorErr(error: APIError<ListDocsCursorError>) {
            description("ListDocsCursorError"),
            display("{:?}", error)
        }
        PaperDocUpdateErr(error: APIError<PaperDocUpdateError>) {
            description("PaperDocUpdateError"),
            display("{:?}", error)
        }
    }
}

impl From<APIError<DocLookupError>> for ErrorKind {
    fn from(error: APIError<DocLookupError>) -> Self {
        ErrorKind::DocLookupErr(error)
    }
}

impl From<APIError<PaperDocCreateError>> for ErrorKind {
    fn from(error: APIError<PaperDocCreateError>) -> Self {
        ErrorKind::PaperDocCreateErr(error)
    }
}

impl From<APIError<ListUsersCursorError>> for ErrorKind {
    fn from(error: APIError<ListUsersCursorError>) -> Self {
        ErrorKind::ListUsersCursorErr(error)
    }
}

impl From<APIError<PaperApiCursorError>> for ErrorKind {
    fn from(error: APIError<PaperApiCursorError>) -> Self {
        ErrorKind::PaperApiCursorErr(error)
    }
}

impl From<APIError<ListDocsCursorError>> for ErrorKind {
    fn from(error: APIError<ListDocsCursorError>) -> Self {
        ErrorKind::ListDocsCursorErr(error)
    }
}

impl From<APIError<PaperDocUpdateError>> for ErrorKind {
    fn from(error: APIError<PaperDocUpdateError>) -> Self {
        ErrorKind::PaperDocUpdateErr(error)
    }
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum DocLookupError {
    InsufficientPermissions,
    DocNotFound,
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PaperDocCreateError {
    InsufficientPermissions,
    ContentMalformed,
    FolderNotFound,
    DocLengthExceeded,
    ImageSizeExceeded,
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ListUsersCursorError {
    InsufficientPermissions,
    DocNotFound,
    CursorError { cursor_error: PaperApiCursorError },
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PaperApiCursorError {
    ExpiredCursor,
    InvalidCursor,
    WrongUserInCursor,
    Reset,
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum ListDocsCursorError {
    CursorError { cursor_error: PaperApiCursorError },
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PaperDocUpdateError {
    InsufficientPermissions,
    DocNotFound,
    ContentMalformed,
    RevisionMismatch,
    DocLengthExceeded,
    ImageSizeExceeded,
    DocArchived,
    DocDeleted,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cursor_error_deserialization() {
        let error_json = r#"{
    "error_summary": "other/...",
    "error": {
        "cursor_error": {".tag": "expired_cursor"}
    }
}"#;
    }
}
