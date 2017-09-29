
pub mod auth;
pub mod paper;
pub mod errors {
    error_chain!{}
}

pub struct Dropbox {
    access_token: String,
}
