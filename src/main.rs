
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
use dropbox::Dropbox;
use dropbox::paper::{ListPaperDocsRequest, ListPaperDocsSortBy};

use reqwest::{Url, Client};

static CLIENT_ID: &'static str = "rfias7v8pyuu83k";
static CLIENT_SECRET: &'static str = "nwq1ootdp0kvv9r";
static REDIRECT_URI: &'static str = "http://localhost";

fn main() {

    let token = run_authorization_flow(CLIENT_ID, REDIRECT_URI, "token").unwrap();
    println!("{:?}", token);
    let auth_ops = AuthOperations::new(CLIENT_ID, CLIENT_SECRET, REDIRECT_URI);
    match token {
        CodeResponse { ref code, .. } => {
            let token = auth_ops.fetch_token(code).unwrap();
            println!("{:?}", token);
        }
        TokenResponse { ref access_token, .. } => {
            let api = Dropbox::new(access_token);
            let list = api.paper_ops.list(&ListPaperDocsRequest {
                filter_by: None,
                sort_by: Some(ListPaperDocsSortBy::modified),
                sort_order: None,
                limit: 100,
            });

            println!("list : {:?}", list);
        }
    }
}
