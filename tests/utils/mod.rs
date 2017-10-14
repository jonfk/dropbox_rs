
extern crate dropbox_rs;
extern crate dotenv;

use std::env;

use dotenv::dotenv;

use dropbox_rs::Dropbox;

pub fn get_dropbox_client() -> Dropbox {
    dotenv().ok();

    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_TOKEN env_var");
    Dropbox::new(&access_code)
}
