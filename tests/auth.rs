extern crate dropbox_rs;
extern crate reqwest;
extern crate uuid;
extern crate serde_json;
extern crate dotenv;

#[path="utils/mod.rs"]
mod utils;

use dropbox_rs::auth::{AuthOperations, TokenFromOAuth1Result, RevokableToken};

use self::utils::get_dropbox_client;

//#[test]
fn test_auth_revoke_token() {
    let client = get_dropbox_client();

    client.revoke_token().expect("error revoking token");
}
