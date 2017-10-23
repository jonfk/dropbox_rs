
extern crate dropbox_rs;
extern crate dotenv;
extern crate env_logger;

use std::env;

use dotenv::dotenv;

use dropbox_rs::Dropbox;

pub fn get_dropbox_client() -> Dropbox {
    dotenv().ok();
    env_logger::init().expect("env_logger init failed");

    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_TOKEN env_var");
    Dropbox::new(&access_code)
}

pub fn get_dropbox_client_revokable() -> Dropbox {
    dotenv().ok();
    env_logger::init().expect("env_logger init failed");

    let access_code = env::var("DROPBOX_TOKEN_REVOKABLE")
        .expect("Couldn't find DROPBOX_TOKEN_REVOKABLE env_var");
    Dropbox::new(&access_code)
}
