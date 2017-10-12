extern crate dropbox_rs;
extern crate reqwest;
extern crate uuid;

use std::env;
use std::io::Read;
use std::ops::Index;

use uuid::Uuid;

use dropbox_rs::paper;
use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsContinueArgs, ListPaperDocsSortBy, ImportFormat, ExportFormat};

#[test]
fn test_paper_create_download_archive_delete() {
    let client = get_dropbox_client();
    let new_uuid = Uuid::new_v4();

    let create_doc = format!(r#"# Test Paper Create {}
## this is h2
hello"#,
                             new_uuid);
    let create_resp = paper::create(&client, ImportFormat::Markdown, None, create_doc)
        .expect("error creating paper doc");
    println!("{:?}", create_resp);
    let doc_id = &create_resp.body.doc_id;

    let download_resp = paper::download(&client, doc_id, ExportFormat::Markdown)
        .expect("error downloading paper doc");

    let mut downloaded_doc = String::new();
    let mut contents = download_resp.content;
    contents.read_to_string(&mut downloaded_doc)
        .expect("error read downloaded content");

    assert!(downloaded_doc.contains(&format!("Test Paper Create {}", new_uuid)));

    paper::archive(&client, doc_id).expect("error archiving doc");

    paper::permanently_delete(&client, doc_id).expect("error permanently deleting doc");
}

fn get_dropbox_client() -> Dropbox {
    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_ACCESS env_var");
    Dropbox::new(&access_code)
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

    let list = paper::list(&client,
                           None,
                           Some(ListPaperDocsSortBy::Modified),
                           None,
                           100)
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
