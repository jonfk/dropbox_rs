
use reqwest::{Url, Client};
use reqwest::header::{Authorization, Bearer};
use serde_json;

use std::io;

use errors::*;
use response::Response;

static BASE_URL: &'static str = "https://api.dropboxapi.com/2/paper/docs/";
const BUFFER_SIZE: usize = 100000;

pub struct PaperOperations {
    access_token: String,
}

impl PaperOperations {
    pub fn new(access_token: &str) -> PaperOperations {
        PaperOperations { access_token: String::from(access_token) }
    }

    pub fn list(&self, request: &ListPaperDocsArgs) -> Result<Response<ListPaperDocsResponse>> {
        let url = Url::parse(BASE_URL)?
            .join("list")?;
        println!("{}", url);

        let client = Client::new()?;
        let mut res = client.post(url)?
            .header(Authorization(Bearer { token: self.access_token.clone() }))
            .json(request)?
            .send()?;

        Ok(Response::try_from(res)?)
    }

    pub fn list_continue(&self,
                         request: &ListPaperDocsContinueArgs)
                         -> Result<Response<ListPaperDocsResponse>> {
        let url = Url::parse(BASE_URL)?
            .join("list/")?
            .join("continue")?;
        println!("{}", url);

        let client = Client::new()?;
        let mut res = client.post(url)?
            .header(Authorization(Bearer { token: self.access_token.clone() }))
            .json(request)?
            .send()?;
        Ok(Response::try_from(res)?)
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
pub struct ListPaperDocsArgs {
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
    pub doc_ids: Vec<String>,
    pub cursor: Cursor,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor {
    pub value: String,
    pub expiration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPaperDocsContinueArgs {
    pub cursor: String,
}
