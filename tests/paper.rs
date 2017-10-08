extern crate dropbox_rs;
extern crate reqwest;

use std::env;
use std::io::Read;

use dropbox_rs::paper;
use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsArgs, ListPaperDocsContinueArgs, ListPaperDocsSortBy,
                        PaperDocCreateArgs, ImportFormat, PaperDocExport, ExportFormat};

#[test]
fn test_paper_list_and_continue() {
    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_ACCESS env_var");
    let client = Dropbox::new(&access_code);

    let list = paper::list(&client,
                           &ListPaperDocsArgs {
                               filter_by: None,
                               sort_by: Some(ListPaperDocsSortBy::Modified),
                               sort_order: None,
                               limit: 100,
                           })
        .expect("error fetching list");

    paper::list_continue(&client,
                         &ListPaperDocsContinueArgs { cursor: list.body.cursor.value })
        .expect("error fetching list/continue");
}

#[test]
fn test_paper_create_download() {
    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_ACCESS env_var");
    let client = Dropbox::new(&access_code);

    let create_doc = r#"# Test Paper Create
## this is h2
hello"#;
    let create_resp = paper::create(&client,
                                    &PaperDocCreateArgs {
                                        import_format: ImportFormat::Markdown,
                                        parent_folder_id: None,
                                    },
                                    create_doc)
        .expect("error creating paper doc");
    println!("{:?}", create_resp);

    let download_resp = paper::download(&client,
                                        &PaperDocExport {
                                            doc_id: create_resp.body.doc_id,
                                            export_format: ExportFormat::Markdown,
                                        })
        .expect("error downloading paper doc");

    let mut downloaded_doc = String::new();
    let mut contents = download_resp.content;
    contents.read_to_string(&mut downloaded_doc)
        .expect("error read downloaded content");

    assert!(downloaded_doc.contains("Test Paper Create"));
}
