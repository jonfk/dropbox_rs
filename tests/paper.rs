
extern crate dropbox_rs;
extern crate reqwest;
extern crate uuid;
extern crate serde_json;
extern crate dotenv;

#[path="utils/mod.rs"]
mod utils;

use std::io::Read;
use std::ops::Index;

use uuid::Uuid;

use dropbox_rs::paper;
use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsSortBy, ImportFormat, ExportFormat, SharingPolicy,
                        SharingPublicPolicyType, PaperDocUpdatePolicy, PaperDocCreateUpdateResult};
use dropbox_rs::paper::users::{MemberSelector, PaperDocPermissionLevel, AddPaperDocUserResult,
                               UserOnPaperDocFilter};

use self::utils::get_dropbox_client;

#[test]
fn test_paper_create_download_archive_delete() {
    let client = get_dropbox_client();

    let (PaperDocCreateUpdateResult { doc_id, .. }, new_uuid) = create_rand_doc(&client);

    let download_resp = paper::download(&client, &doc_id, ExportFormat::Markdown)
        .expect("error downloading paper doc");

    let mut downloaded_doc = String::new();
    let mut contents = download_resp.content;
    contents.read_to_string(&mut downloaded_doc)
        .expect("error read downloaded content");

    assert!(downloaded_doc.contains(&format!("Test Paper Create {}", new_uuid)));

    paper::archive(&client, &doc_id).expect("error archiving doc");

    paper::permanently_delete(&client, &doc_id).expect("error permanently deleting doc");
}

fn create_rand_doc(client: &Dropbox) -> (PaperDocCreateUpdateResult, Uuid) {
    let new_uuid = Uuid::new_v4();
    let create_doc = format!(r#"# Test Paper Create {}
## this is h2
hello"#,
                             new_uuid);
    let create_resp = paper::create(client, ImportFormat::Markdown, None, create_doc)
        .expect("error creating paper doc");
    (create_resp.body, new_uuid)
}

#[test]
fn test_list_folder_users() {
    let client = get_dropbox_client();

    let list = paper::list(&client,
                           None,
                           Some(ListPaperDocsSortBy::Modified),
                           None,
                           100)
        .expect("error fetching list");
    let doc_id = list.body.doc_ids.index(0);
    let folder_users_list = paper::list_folder_users(&client, doc_id, 2)
        .expect("failed list folder users");

    println!("{:?}", folder_users_list);
}

#[test]
fn test_paper_list_and_continue() {
    let client = get_dropbox_client();

    let list = paper::list(&client, None, Some(ListPaperDocsSortBy::Modified), None, 10)
        .expect("error fetching list");

    paper::list_continue(&client, &list.body.cursor.value).expect("error fetching list/continue");
}

#[test]
fn test_list_get_folder_info() {
    let client = get_dropbox_client();

    let list = paper::list(&client,
                           None,
                           Some(ListPaperDocsSortBy::Modified),
                           None,
                           100)
        .expect("error fetching list");
    let doc_id = list.body.doc_ids.index(0);
    let folder_info = paper::get_folder_info(&client, doc_id).expect("error getting folder info");
    println!("folder_info: {:?}", folder_info);
}

#[test]
fn test_get_set_sharing_policy() {
    let client = get_dropbox_client();
    let (PaperDocCreateUpdateResult { doc_id, .. }, _) = create_rand_doc(&client);

    let sharing_policy = paper::get_sharing_policy(&client, &doc_id)
        .expect("error getting sharing policy");
    println!("{:?}", sharing_policy);

    let expected_public_sharing_policy =
        Some(SharingPublicPolicyType::PeopleWithLinkCanViewAndComment);
    paper::set_sharing_policy(&client, &doc_id, expected_public_sharing_policy, None)
        .expect("error setting sharing policy");

    let SharingPolicy { public_sharing_policy, team_sharing_policy } =
        paper::get_sharing_policy(&client, &doc_id).expect("error getting sharing policy").body;

    assert_eq!(public_sharing_policy, expected_public_sharing_policy);

    paper::permanently_delete(&client, &doc_id).expect("error deleting doc");
}

#[test]
fn test_update() {
    let client = get_dropbox_client();

    let (PaperDocCreateUpdateResult { doc_id, revision, .. }, _) = create_rand_doc(&client);

    let new_uuid = Uuid::new_v4();

    let update_content = format!(r#"hello updated with this {}"#, new_uuid);

    let update_result = paper::update(&client,
                                      &doc_id,
                                      PaperDocUpdatePolicy::OverwriteAll,
                                      revision,
                                      ImportFormat::PlainText,
                                      update_content.clone())
        .expect("error updating doc");

    let download_resp = paper::download(&client, &doc_id, ExportFormat::Markdown)
        .expect("error downloading paper doc");

    let mut downloaded_doc = String::new();
    let mut contents = download_resp.content;
    contents.read_to_string(&mut downloaded_doc)
        .expect("error read downloaded content");

    assert!(downloaded_doc.contains(&update_content));

    paper::permanently_delete(&client, &doc_id).expect("error permanently deleting doc");
}

#[test]
fn test_users_add_list_remove() {
    let client = get_dropbox_client();

    let (PaperDocCreateUpdateResult { doc_id, .. }, _) = create_rand_doc(&client);

    let member_selector = MemberSelector::Email { email: "jfokkan@gmail.com".to_owned() };

    let users_add_result = paper::users_add(&client, doc_id.as_str())
        .custom_message("hello jfokkan from dropbox_rs")
        .add_member(&member_selector, &PaperDocPermissionLevel::Edit)
        .send()
        .expect("error adding users");
    println!("{:?}", users_add_result);

    let users_list = paper::users_list(&client, &doc_id, 10, UserOnPaperDocFilter::Shared)
        .expect("error listing users");

    paper::users_remove(&client, &doc_id, &member_selector).expect("error removing user");

    let users_list_after_remove =
        paper::users_list(&client, &doc_id, 10, UserOnPaperDocFilter::Shared)
            .expect("error listing users");

    paper::permanently_delete(&client, &doc_id).expect("error permanently deleting doc");

    assert_eq!(users_add_result.body[0].member, member_selector);
    assert_eq!(users_add_result.body[0].result,
               AddPaperDocUserResult::Success);

    assert_eq!(users_list.body.invitees[0].invitee.email,
               "jfokkan@gmail.com");
    assert_eq!(users_list_after_remove.body.invitees.len(), 0);
}
