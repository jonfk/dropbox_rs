use std::io::Read;

use reqwest::Url;
use reqwest::Body;

use errors::*;
use http::{Response, ContentResponse};
use http::{RPCClient, ContentDownloadClient, ContentUploadClient};

static BASE_URL: &'static str = "https://api.dropboxapi.com/2/paper/docs/";

pub fn archive<T: RPCClient>(client: &T, doc_id: &str) -> Result<Response<()>> {
    let url = Url::parse(BASE_URL)?
        .join("archive")?;
    println!("{}", url);
    let request = RefPaperDoc { doc_id: doc_id.to_owned() };

    client.rpc_request(url, request)
}

pub fn create<T: ContentUploadClient, C: Into<Body>>
    (client: &T,
     import_format: ImportFormat,
     parent_folder_id: Option<&str>,
     content: C)
     -> Result<Response<PaperDocCreateUpdateResult>> {

    let url = Url::parse(BASE_URL)?
        .join("create")?;
    println!("{}", url);

    client.content_upload_request(url,
                                  PaperDocCreateArgs {
                                      import_format: import_format,
                                      parent_folder_id: parent_folder_id.map(|x| x.to_owned()),
                                  },
                                  content)
}

pub fn download<C, T: ContentDownloadClient<C>>
    (client: &T,
     doc_id: &str,
     export_format: ExportFormat)
     -> Result<ContentResponse<PaperDocExportResult, C>>
    where C: Read
{
    let url = Url::parse(BASE_URL)?
        .join("download")?;
    println!("{}", url);
    client.content_download(url,
                            PaperDocExport {
                                doc_id: doc_id.to_owned(),
                                export_format: export_format,
                            })
}

pub fn list_folder_users<T: RPCClient>(client: &T,
                                       doc_id: &str,
                                       limit: i32)
                                       -> Result<Response<ListUsersOnFolderResponse>> {
    let url = Url::parse(BASE_URL)?.join("folder_users/list")?;
    println!("{}", url);
    client.rpc_request(url,
                       &ListUsersOnFolderArgs {
                           doc_id: doc_id.to_owned(),
                           limit: limit,
                       })
}

pub fn list_folder_users_continue<T: RPCClient>(client: &T,
                                                doc_id: &str,
                                                cursor: &str)
                                                -> Result<Response<ListUsersOnFolderResponse>> {
    let url = Url::parse(BASE_URL)?.join("folder_users/list/continue")?;
    println!("{}", url);
    client.rpc_request(url,
                       &ListUsersOnFolderContinueArgs {
                           doc_id: doc_id.to_owned(),
                           cursor: cursor.to_owned(),
                       })
}

// TODO implement a builder for optional parameters
pub fn list<T: RPCClient>(client: &T,
                          filter_by: Option<ListPaperDocsFilterBy>,
                          sort_by: Option<ListPaperDocsSortBy>,
                          sort_order: Option<ListPaperDocsSortOrder>,
                          limit: usize)
                          -> Result<Response<ListPaperDocsResponse>> {
    let url = Url::parse(BASE_URL)?
        .join("list")?;
    println!("{}", url);

    client.rpc_request(url,
                       &ListPaperDocsArgs {
                           filter_by: filter_by,
                           sort_by: sort_by,
                           sort_order: sort_order,
                           limit: limit,
                       })
}

pub fn list_continue<T: RPCClient>(client: &T,
                                   request: &ListPaperDocsContinueArgs)
                                   -> Result<Response<ListPaperDocsResponse>> {
    let url = Url::parse(BASE_URL)?
        .join("list/")?
        .join("continue")?;
    println!("{}", url);

    client.rpc_request(url, request)
}


/**
 * archive
 **/
#[derive(Debug,Serialize,Deserialize)]
pub struct RefPaperDoc {
    pub doc_id: String,
}

/**
 * create
 **/
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PaperDocCreateArgs {
    pub import_format: ImportFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportFormat {
    Html,
    Markdown,
    PlainText,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct PaperDocCreateUpdateResult {
    pub doc_id: String,
    pub revision: i64,
    pub title: String,
}

/**
 * download
 **/
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PaperDocExport {
    pub doc_id: String,
    pub export_format: ExportFormat,
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Html,
    Markdown,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PaperDocExportResult {
    pub owner: String,
    pub title: String,
    pub revision: i64,
    pub mime_type: String,
}

/**
 * Folder Users
 **/
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ListUsersOnFolderArgs {
    pub doc_id: String,
    pub limit: i32,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ListUsersOnFolderContinueArgs {
    pub doc_id: String,
    pub cursor: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ListUsersOnFolderResponse {
    pub invitees: Vec<InviteeInfo>,
    pub users: Vec<UserInfo>,
    pub cursor: Cursor,
    pub has_more: bool,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InviteeInfo {
    pub email: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct UserInfo {
    pub account_id: String,
    pub same_team: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
}


/**
 * List
 **/
#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListPaperDocsFilterBy {
    DocsAccessed,
    DocsCreated,
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListPaperDocsSortBy {
    Accessed,
    Modified,
    Created,
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListPaperDocsSortOrder {
    Ascending,
    Descending,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub value: String,
    pub expiration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPaperDocsContinueArgs {
    pub cursor: String,
}
