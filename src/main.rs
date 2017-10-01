
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

mod dropbox;

use dropbox::auth::AuthOperations;
use dropbox::auth::AuthorizationResponse::{CodeResponse, TokenResponse};
use dropbox::auth::{AuthTokenRequest, AuthorizationResponse};
use dropbox::Dropbox;
use dropbox::paper::{ListPaperDocsRequest, ListPaperDocsSortBy};
use dropbox::errors::*;

use reqwest::{Url, Client};

use std::io::{self, Write};

static CLIENT_ID: &'static str = "rfias7v8pyuu83k";
static CLIENT_SECRET: &'static str = "nwq1ootdp0kvv9r";
static REDIRECT_URI: &'static str = "http://localhost";

fn main() {
    let test = run();
    println!("{:?}", test);
    if let Err(ref e) = test {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run_authorization_flow(client_id: &str,
                          redirect_uri: &str,
                          response_type: &str)
                          -> Option<AuthorizationResponse> {
    println!("Please visit the following url and authorize this app {}",
             dropbox::auth::build_authorization_uri(client_id, redirect_uri, response_type));
    print!("Paste  redirect url here : ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    dropbox::auth::parse_authorization_response(&buffer)
}

fn run() -> Result<()> {
    let token = run_authorization_flow(CLIENT_ID, REDIRECT_URI, "token").unwrap();
    let auth_ops = AuthOperations::new(CLIENT_ID, CLIENT_SECRET, REDIRECT_URI);
    match token {
        CodeResponse { ref code, .. } => {
            let token = auth_ops.fetch_token(code)?;
            println!("{:?}", token);
            Ok(())
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
            Ok(())
        }
    }
}
