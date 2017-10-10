extern crate dropbox_rs;
extern crate reqwest;

use std::env;
use std::io::Read;
use std::ops::Index;

use dropbox_rs::paper;
use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsContinueArgs, ListPaperDocsSortBy, ImportFormat, ExportFormat};

#[test]
fn test_paper_create_download() {
    let client = get_dropbox_client();

    let create_doc = r#"# Test Paper Create
## this is h2
hello"#;
    let create_resp = paper::create(&client, ImportFormat::Markdown, None, create_doc)
        .expect("error creating paper doc");
    println!("{:?}", create_resp);

    let download_resp = paper::download(&client, &create_resp.body.doc_id, ExportFormat::Markdown)
        .expect("error downloading paper doc");

    let mut downloaded_doc = String::new();
    let mut contents = download_resp.content;
    contents.read_to_string(&mut downloaded_doc)
        .expect("error read downloaded content");

    assert!(downloaded_doc.contains("Test Paper Create"));
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

    paper::list_continue(&client,
                         &ListPaperDocsContinueArgs { cursor: list.body.cursor.value })
        .expect("error fetching list/continue");
}
