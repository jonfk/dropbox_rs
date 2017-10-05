extern crate dropbox_rs;

use std::env;

use dropbox_rs::paper;
use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsArgs, ListPaperDocsContinueArgs, ListPaperDocsSortBy,
                        PaperDocCreateArgs, ImportFormat};

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

    let list_continue =
        paper::list_continue(&client,
                             &ListPaperDocsContinueArgs { cursor: list.body.cursor.value })
            .expect("error fetching list/continue");
}

#[test]
fn test_paper_create() {
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
}
