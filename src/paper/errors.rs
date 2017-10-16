
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
