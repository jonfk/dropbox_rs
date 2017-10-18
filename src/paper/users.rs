
use serde::{Serialize, Serializer};

use reqwest::Url;

use paper::errors::*;
use http::{Response, ResponseWithErr};
use http::RPCClient;

/**
 * add users
 **/
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum MemberSelector {
    DropboxId { dropbox_id: String },
    Email { email: String },
}
#[derive(PartialEq,Eq,Debug,Copy,Clone,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum PaperDocPermissionLevel {
    Edit,
    ViewAndComment,
}
impl Serialize for PaperDocPermissionLevel {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            PaperDocPermissionLevel::Edit => {
                serializer.serialize_unit_variant("PaperDocPermissionLevel", 0, "edit")
            }
            PaperDocPermissionLevel::ViewAndComment => {
                serializer.serialize_unit_variant("PaperDocPermissionLevel", 1, "view_and_comment")
            }
        }
    }
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddMember {
    pub member: MemberSelector,
    pub permission_level: PaperDocPermissionLevel,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddPaperDocUser {
    pub doc_id: String,
    pub members: Vec<AddMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    pub quiet: bool,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
pub enum AddPaperDocUserResult {
    Success,
    UnknownError,
    SharingOutsideTeamDisabled,
    DailyLimitReached,
    UserIsOwner,
    FailedUserDataRetrieval,
    PermissionAlreadyGranted,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct AddPaperDocUserMemberResult {
    pub member: MemberSelector,
    pub result: AddPaperDocUserResult,
}

pub struct AddPaperDocUserRequestBuilder<T> {
    client: T,
    doc_id: String,
    members: Vec<AddMember>,
    custom_message: Option<String>,
    quiet: bool,
}

impl<T> AddPaperDocUserRequestBuilder<T>
    where T: RPCClient + Clone
{
    pub fn new(client: &T, doc_id: &str) -> AddPaperDocUserRequestBuilder<T> {
        AddPaperDocUserRequestBuilder {
            client: client.clone(),
            doc_id: doc_id.to_owned(),
            members: Vec::new(),
            custom_message: None,
            quiet: false,
        }
    }

    pub fn add_member(&mut self,
                      member: &MemberSelector,
                      permission_level: &PaperDocPermissionLevel)
                      -> &mut AddPaperDocUserRequestBuilder<T> {
        self.members.push(AddMember {
            member: member.clone(),
            permission_level: permission_level.clone(),
        });
        self
    }

    pub fn quiet(&mut self, quiet: bool) -> &mut AddPaperDocUserRequestBuilder<T> {
        self.quiet = quiet;
        self
    }
    pub fn custom_message(&mut self,
                          custom_message: &str)
                          -> &mut AddPaperDocUserRequestBuilder<T> {
        self.custom_message = Some(custom_message.to_owned());;
        self
    }

    // pub fn send(&self) -> Result<Response<Vec<AddPaperDocUserMemberResult>>> {
    //     let url = Url::parse(super::BASE_URL)?.join("users/add")?;
    //     let resp_with_err = self.client.rpc_request(url,
    //                                                 AddPaperDocUser {
    //                                                     doc_id: self.doc_id.clone(),
    //                                                     members: self.members.clone(),
    //                                                     custom_message: self.custom_message.clone(),
    //                                                     quiet: self.quiet,
    //                                                 });
    //     match resp_with_err {
    //         ResponseWithErr::Ok(r) => Ok(r),
    //         ResponseWithErr::Err(e) => Err(e),
    //     }
    // }
}

/**
 * list
 **/
#[derive(PartialEq,Eq,Debug,Copy,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserOnPaperDocFilter {
    Visited,
    Shared,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct ListUsersOnPaperDocArgs {
    pub doc_id: String,
    pub limit: i32,
    pub filter_by: UserOnPaperDocFilter,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct InviteeInfo {
    pub email: String,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct InviteeInfoWithPermissionLevel {
    pub invitee: InviteeInfo,
    pub permission_level: PaperDocPermissionLevel,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct UserInfo {
    pub account_id: String,
    pub same_team: bool,
    pub team_member_id: Option<String>,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct UserInfoWithPermissionLevel {
    pub user: UserInfo,
    pub permission_level: PaperDocPermissionLevel,
}
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct ListUsersOnPaperDocResponse {
    pub invitees: Vec<InviteeInfoWithPermissionLevel>,
    pub users: Vec<UserInfoWithPermissionLevel>,
    pub doc_owner: UserInfo,
    pub cursor: super::Cursor,
    pub has_more: bool,
}

#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct ListUsersOnPaperDocContinueArgs {
    pub doc_id: String,
    pub cursor: String,
}

/**
 * remove
 **/
#[derive(PartialEq,Eq,Debug,Clone,Serialize,Deserialize)]
pub struct RemovePaperDocUser {
    pub doc_id: String,
    pub member: MemberSelector,
}
