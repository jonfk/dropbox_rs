extern crate dropbox_rs;

use std::env;

use dropbox_rs::paper;
use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsArgs, ListPaperDocsContinueArgs, ListPaperDocsSortBy};

#[test]
fn test_paper_list_and_continue() {
    let access_code = env::var("DROPBOX_TOKEN").expect("Couldn't find DROPBOX_ACCESS env_var");

    let client = Dropbox::new(&access_code);
    let list = paper::list(&client,
                           &ListPaperDocsArgs {
                               filter_by: None,
                               sort_by: Some(ListPaperDocsSortBy::modified),
                               sort_order: None,
                               limit: 100,
                           })
        .expect("error fetching list");

    let list_continue =
        paper::list_continue(&client,
                             &ListPaperDocsContinueArgs { cursor: list.body.cursor.value })
            .expect("error fetching list/continue");
}
