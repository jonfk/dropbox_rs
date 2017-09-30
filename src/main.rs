
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

mod dropbox;

use dropbox::auth::{run_authorization_flow, AuthOperations};
use dropbox::auth::AuthorizationResponse::{CodeResponse, TokenResponse};
use dropbox::auth::AuthTokenRequest;

use reqwest::{Url, Client};

static CLIENT_ID: &'static str = "rfias7v8pyuu83k";
static CLIENT_SECRET: &'static str = "nwq1ootdp0kvv9r";
static REDIRECT_URI: &'static str = "http://localhost";

fn main() {

    let token = run_authorization_flow(CLIENT_ID, REDIRECT_URI, "code").unwrap();
    println!("{:?}", token);
    let auth_ops = AuthOperations {
        client_id: String::from(CLIENT_ID),
        client_secret: String::from(CLIENT_SECRET),
        redirect_uri: String::from(REDIRECT_URI),
    };
    match token {
        CodeResponse { ref code, .. } => {
            let token = auth_ops.fetch_token(code).unwrap();
            println!("{:?}", token);
        }
        _ => {}
    }
}
