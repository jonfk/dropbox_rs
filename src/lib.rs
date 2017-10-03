
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

const BUFFER_SIZE: usize = 100000;

pub mod auth;
pub mod paper;
pub mod response;
pub mod errors;

use paper::PaperOperations;

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
