pub mod users;
pub mod errors;

use serde::{Serialize, Serializer};
use reqwest::Url;
use reqwest::Body;
use reqwest::Response as ReqwestResponse;

use std::rc::Rc;

use self::errors::*;
use http::{Response, ContentResponse};
use http::{ResponseWithErr, ContentResponseWithErr};
use http::{RPCClient, ContentDownloadClient, ContentUploadClient};

use self::users::{AddPaperDocUserRequestBuilder, UserOnPaperDocFilter, ListUsersOnPaperDocResponse,
                  ListUsersOnPaperDocArgs, ListUsersOnPaperDocContinueArgs, RemovePaperDocUser,
                  MemberSelector};

static BASE_URL: &'static str = "https://api.dropboxapi.com/2/paper/docs/";

#[derive(Debug,Clone)]
pub struct Paper {
    access_token: Rc<String>,
}

impl ::http::HasAccessToken for Paper {
    fn access_token(&self) -> &str {
        self.access_token.as_str()
    }
}

impl Paper {
    pub fn new(access_token: Rc<String>) -> Paper {
        Paper { access_token: access_token.clone() }
    }

    pub fn archive(&self, doc_id: &str) -> Result<Response<()>> {
        let url = Url::parse(BASE_URL)?
            .join("archive")?;
        println!("{}", url);
        let request = RefPaperDoc { doc_id: doc_id.to_owned() };
        let resp_w_err: ResponseWithErr<_, DocLookupError> = self.rpc_request(url, request)?;

        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn create<C: Into<Body>>(&self,
                                 import_format: ImportFormat,
                                 parent_folder_id: Option<&str>,
                                 content: C)
                                 -> Result<Response<PaperDocCreateUpdateResult>> {
        let url = Url::parse(BASE_URL)?
            .join("create")?;
        println!("{}", url);

        let resp_w_err = self.content_upload_request(url,
                                    PaperDocCreateArgs {
                                        import_format: import_format,
                                        parent_folder_id: parent_folder_id.map(|x| x.to_owned()),
                                    },
                                    content)?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::PaperDocCreateErr(e).into()),
        }
    }

    pub fn download(&self,
                    doc_id: &str,
                    export_format: ExportFormat)
                    -> Result<ContentResponse<PaperDocExportResult, ReqwestResponse>> {
        let url = Url::parse(BASE_URL)?
            .join("download")?;
        println!("{}", url);
        let resp_w_err = self.content_download(url,
                              PaperDocExport {
                                  doc_id: doc_id.to_owned(),
                                  export_format: export_format,
                              })?;
        match resp_w_err {
            ContentResponseWithErr::Ok(r) => Ok(r),
            ContentResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn list_folder_users(&self,
                             doc_id: &str,
                             limit: i32)
                             -> Result<Response<ListUsersOnFolderResponse>> {
        let url = Url::parse(BASE_URL)?.join("folder_users/list")?;
        println!("{}", url);
        let resp_w_err = self.rpc_request(url,
                         &ListUsersOnFolderArgs {
                             doc_id: doc_id.to_owned(),
                             limit: limit,
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn list_folder_users_continue(&self,
                                      doc_id: &str,
                                      cursor: &str)
                                      -> Result<Response<ListUsersOnFolderResponse>> {
        let url = Url::parse(BASE_URL)?.join("folder_users/list/continue")?;
        println!("{}", url);
        let resp_w_err = self.rpc_request(url,
                         &ListUsersOnFolderContinueArgs {
                             doc_id: doc_id.to_owned(),
                             cursor: cursor.to_owned(),
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::ListUsersCursorErr(e).into()),
        }
    }

    pub fn get_folder_info(&self, doc_id: &str) -> Result<Response<FoldersContainingPaperDoc>> {
        let url = Url::parse(BASE_URL)?.join("get_folder_info")?;
        println!("{}", url);
        let resp_w_err = self.rpc_request(url, &RefPaperDoc { doc_id: doc_id.to_owned() })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    // TODO implement a builder for optional parameters?
    pub fn list(&self,
                filter_by: Option<ListPaperDocsFilterBy>,
                sort_by: Option<ListPaperDocsSortBy>,
                sort_order: Option<ListPaperDocsSortOrder>,
                limit: usize)
                -> Result<Response<ListPaperDocsResponse>> {
        let url = Url::parse(BASE_URL)?
            .join("list")?;
        println!("{}", url);

        let resp_w_err: ResponseWithErr<_, ()> = self.rpc_request(url,
                         &ListPaperDocsArgs {
                             filter_by: filter_by,
                             sort_by: sort_by,
                             sort_order: sort_order,
                             limit: limit,
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(_) => {
                unreachable!("paper: https://api.dropboxapi.com/2/paper/docs/list should not \
                              return errors")
            }
        }
    }

    pub fn list_continue(&self, cursor: &str) -> Result<Response<ListPaperDocsResponse>> {
        let url = Url::parse(BASE_URL)?
            .join("list/")?
            .join("continue")?;
        println!("{}", url);

        let resp_w_err = self.rpc_request(url,
                         &ListPaperDocsContinueArgs { cursor: cursor.to_owned() })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::ListDocsCursorErr(e).into()),
        }
    }

    pub fn permanently_delete(&self, doc_id: &str) -> Result<Response<()>> {
        let url = Url::parse(BASE_URL)?
            .join("permanently_delete")?;
        println!("{}", url);

        let resp_w_err = self.rpc_request(url, &RefPaperDoc { doc_id: doc_id.to_owned() })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn get_sharing_policy(&self, doc_id: &str) -> Result<Response<SharingPolicy>> {
        let url = Url::parse(BASE_URL)?
            .join("sharing_policy/get")?;
        println!("{}", url);

        let resp_w_err = self.rpc_request(url, &RefPaperDoc { doc_id: doc_id.to_owned() })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn set_sharing_policy(&self,
                              doc_id: &str,
                              public_sharing_policy: Option<SharingPublicPolicyType>,
                              team_sharing_policy: Option<SharingTeamPolicyType>)
                              -> Result<Response<()>> {
        let url = Url::parse(BASE_URL)?
            .join("sharing_policy/set")?;
        println!("{}", url);

        let resp_w_err = self.rpc_request(url,
                         &PaperDocSharingPolicy {
                             doc_id: doc_id.to_owned(),
                             sharing_policy: SharingPolicy {
                                 public_sharing_policy: public_sharing_policy,
                                 team_sharing_policy: team_sharing_policy,
                             },
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn update<C: Into<Body>>(&self,
                                 doc_id: &str,
                                 doc_update_policy: PaperDocUpdatePolicy,
                                 revision: i64,
                                 import_format: ImportFormat,
                                 content: C)
                                 -> Result<Response<PaperDocCreateUpdateResult>> {

        let url = Url::parse(BASE_URL)?
            .join("update")?;
        println!("{}", url);

        let resp_w_err = self.content_upload_request(url,
                                    PaperDocUpdateArgs {
                                        doc_id: doc_id.to_owned(),
                                        doc_update_policy: doc_update_policy,
                                        revision: revision,
                                        import_format: import_format,
                                    },
                                    content)?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::PaperDocUpdateErr(e).into()),
        }
    }

    pub fn users_add(&self, doc_id: &str) -> AddPaperDocUserRequestBuilder<Paper> {
        AddPaperDocUserRequestBuilder::new(self, doc_id)
    }

    pub fn users_list(&self,
                      doc_id: &str,
                      limit: i32,
                      filter_by: UserOnPaperDocFilter)
                      -> Result<Response<ListUsersOnPaperDocResponse>> {
        let url = Url::parse(BASE_URL)?.join("users/list")?;
        let resp_w_err = self.rpc_request(url,
                         &ListUsersOnPaperDocArgs {
                             doc_id: doc_id.to_owned(),
                             limit: limit,
                             filter_by: filter_by,
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }

    pub fn users_list_continue(&self,
                               doc_id: &str,
                               cursor: &str)
                               -> Result<Response<ListUsersOnPaperDocResponse>> {
        let url = Url::parse(BASE_URL)?.join("users/list/continue")?;
        let resp_w_err = self.rpc_request(url,
                         &ListUsersOnPaperDocContinueArgs {
                             doc_id: doc_id.to_owned(),
                             cursor: cursor.to_owned(),
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::ListUsersCursorErr(e).into()),
        }
    }

    pub fn users_remove(&self, doc_id: &str, member: &MemberSelector) -> Result<Response<()>> {
        let url = Url::parse(BASE_URL)?.join("users/remove")?;
        let resp_w_err = self.rpc_request(url,
                         &RemovePaperDocUser {
                             doc_id: doc_id.to_owned(),
                             member: member.clone(),
                         })?;
        match resp_w_err {
            ResponseWithErr::Ok(r) => Ok(r),
            ResponseWithErr::Err(e) => Err(ErrorKind::DocLookupErr(e).into()),
        }
    }
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

#[derive(PartialEq,Eq,Debug,Clone,Copy,Serialize,Deserialize)]
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
 * folder info
 **/
#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum FolderSharingPolicyType {
    Team,
    InviteOnly,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Folder {
    id: String,
    name: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct FoldersContainingPaperDoc {
    pub folder_sharing_policy_type: FolderSharingPolicyType,
    pub folders: Vec<Folder>,
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

#[derive(PartialEq,Eq,Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub value: String,
    pub expiration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPaperDocsContinueArgs {
    pub cursor: String,
}

/**
 * Sharing Policy
 **/
#[derive(PartialEq,Eq,Debug,Copy,Clone,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharingPublicPolicyType {
    PeopleWithLinkCanEdit,
    PeopleWithLinkCanViewAndComment,
    InviteOnly,
    Disabled,
}

impl Serialize for SharingPublicPolicyType {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            SharingPublicPolicyType::PeopleWithLinkCanEdit => {
                serializer.serialize_unit_variant("SharingPublicPolicyType",
                                                  0,
                                                  "people_with_link_can_edit")
            }
            SharingPublicPolicyType::PeopleWithLinkCanViewAndComment => {
                serializer.serialize_unit_variant("SharingPublicPolicyType",
                                                  1,
                                                  "people_with_link_can_view_and_comment")
            }
            SharingPublicPolicyType::InviteOnly => {
                serializer.serialize_unit_variant("SharingPublicPolicyType", 2, "invite_only")
            }
            SharingPublicPolicyType::Disabled => {
                serializer.serialize_unit_variant("SharingPublicPolicyType", 3, "disabled")
            }
        }
    }
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum SharingTeamPolicyType {
    PeopleWithLinkCanEdit,
    PeopleWithLinkCanViewAndComment,
    InviteOnly,
}

impl Serialize for SharingTeamPolicyType {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            SharingTeamPolicyType::PeopleWithLinkCanEdit => {
                serializer.serialize_unit_variant("SharingTeamPolicyType",
                                                  0,
                                                  "people_with_link_can_edit")
            }
            SharingTeamPolicyType::PeopleWithLinkCanViewAndComment => {
                serializer.serialize_unit_variant("SharingTeamPolicyType",
                                                  1,
                                                  "people_with_link_can_view_and_comment")
            }
            SharingTeamPolicyType::InviteOnly => {
                serializer.serialize_unit_variant("SharingTeamPolicyType", 2, "invite_only")
            }
        }
    }
}

#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
pub struct SharingPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_sharing_policy: Option<SharingPublicPolicyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_sharing_policy: Option<SharingTeamPolicyType>,
}

#[derive(PartialEq,Eq,Debug,Clone,Serialize)]
struct PaperDocSharingPolicy {
    doc_id: String,
    sharing_policy: SharingPolicy,
}

/**
 * Update
 **/
#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaperDocUpdatePolicy {
    Append,
    Prepend,
    OverwriteAll,
}

#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct PaperDocUpdateArgs {
    pub doc_id: String,
    pub doc_update_policy: PaperDocUpdatePolicy,
    pub revision: i64,
    pub import_format: ImportFormat,
}
