#![recursion_limit="1024"]

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
pub mod http;

use std::borrow::Cow;

use paper::Paper;

#[derive(Clone)]
pub struct Dropbox {
    access_token: String,
    paper: Paper,
}

impl Dropbox {
    pub fn new(access_token: &str) -> Dropbox {
        let dropbox_access_token = access_token.to_owned();
        Dropbox {
            access_token: dropbox_access_token,
            paper: Paper::new(access_token),
        }
    }

    pub fn paper(&self) -> &Paper {
        &self.paper
    }
}
