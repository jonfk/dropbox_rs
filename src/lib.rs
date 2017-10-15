
extern crate reqwest;
#[macro_use]
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

pub mod auth;
pub mod paper;
pub mod errors;
pub mod http;

#[derive(Clone)]
pub struct Dropbox {
    access_token: String,
}

impl Dropbox {
    pub fn new(access_token: &str) -> Dropbox {
        Dropbox { access_token: String::from(access_token) }
    }
}
