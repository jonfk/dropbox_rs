
extern crate dropbox_rs;
extern crate dotenv;
extern crate env_logger;

use std::env;

use dotenv::dotenv;

use dropbox_rs::Dropbox;

pub fn get_dropbox_client() -> Dropbox {
    dotenv().ok();
    // Ignore error on init since it might be called several times during tests
    let _ = env_logger::init();

    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_TOKEN env_var");
    Dropbox::new(&access_code)
}

pub fn get_dropbox_client_revokable() -> Dropbox {
    dotenv().ok();
    // Ignore error on init since it might be called several times during tests
    let _ = env_logger::init();

    let access_code = env::var("DROPBOX_TOKEN_REVOKABLE")
        .expect("Couldn't find DROPBOX_TOKEN_REVOKABLE env_var");
    Dropbox::new(&access_code)
}
