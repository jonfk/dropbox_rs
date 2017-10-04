
use serde_json;
use reqwest::Url;
use reqwest::header::{Authorization, Bearer};
use reqwest::Body;

use std::io;

use super::Dropbox;
use errors::*;
use http::Response;
use http::Client;

static BASE_URL: &'static str = "https://api.dropboxapi.com/2/paper/docs/";

pub fn list<T: Client>(client: &T,
                       request: &ListPaperDocsArgs)
                       -> Result<Response<ListPaperDocsResponse>> {
    let url = Url::parse(BASE_URL)?
        .join("list")?;
    println!("{}", url);

    client.rpc_request(url, request)
}

pub fn list_continue<T: Client>(client: &T,
                                request: &ListPaperDocsContinueArgs)
                                -> Result<Response<ListPaperDocsResponse>> {
    let url = Url::parse(BASE_URL)?
        .join("list/")?
        .join("continue")?;
    println!("{}", url);

    client.rpc_request(url, request)
}

pub fn archive<T: Client>(client: &T, doc_id: &str) -> Result<Response<()>> {
    let url = Url::parse(BASE_URL)?
        .join("archive")?;
    println!("{}", url);
    let request = RefPaperDoc { doc_id: doc_id.to_owned() };

    client.rpc_request(url, request)
}

pub fn create<T: Client, C: Into<Body>>(client: &T,
                                        request: &PaperDocCreateArgs,
                                        content: C)
                                        -> Result<Response<PaperDocCreateUpdateResult>> {

    let url = Url::parse(BASE_URL)?
        .join("create")?;
    println!("{}", url);

    client.content_upload_request(url, request, content)
}

/**
 * archive
 **/
#[derive(Debug,Serialize,Deserialize)]
pub struct RefPaperDoc {
    doc_id: String,
}

/**
 * create
 **/
#[derive(Debug,Serialize,Deserialize)]
pub struct PaperDocCreateArgs {
    import_format: String,
    parent_folder_id: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct PaperDocCreateUpdateResult {
    doc_id: String,
    revision: i64,
    title: String,
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
