
use reqwest::{Url, Client};
use reqwest::header::{Authorization, Bearer};
use serde_json;

use std::io;

use dropbox::errors::*;

static BASE_URL: &'static str = "https://api.dropboxapi.com/2/paper/docs/";
const BUFFER_SIZE: usize = 100000;

pub struct PaperOperations {
    access_token: String,
}

impl PaperOperations {
    pub fn new(access_token: &str) -> PaperOperations {
        PaperOperations { access_token: String::from(access_token) }
    }
    pub fn list(&self, request: &ListPaperDocsRequest) -> Result<ListPaperDocsResponse> {
        let url = Url::parse(BASE_URL)?
            .join("list")?;
        println!("{}", url);

        let client = Client::new()?;
        let mut res = client.post(url)?
            .header(Authorization(Bearer { token: self.access_token.clone() }))
            .json(request)?
            .send()?;

        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        io::copy(&mut res, &mut buf)?;
        println!("{}", String::from_utf8(buf.clone())?);

        Ok(serde_json::from_slice(&buf)?)
    }
}

/**
 * List
 **/
#[derive(Debug,Serialize,Deserialize)]
pub enum ListPaperDocsFilterBy {
    docs_accessed,
    docs_created,
}

#[derive(Debug,Serialize,Deserialize)]
pub enum ListPaperDocsSortBy {
    accessed,
    modified,
    created,
}

#[derive(Debug,Serialize,Deserialize)]
pub enum ListPaperDocsSortOrder {
    ascending,
    descending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPaperDocsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<ListPaperDocsFilterBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ListPaperDocsSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<ListPaperDocsSortOrder>,
    pub limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPaperDocsResponse {
    doc_ids: Vec<String>,
    cursor: Cursor,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cursor {
    value: String,
    expiration: String,
}
