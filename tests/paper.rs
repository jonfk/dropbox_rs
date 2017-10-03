extern crate dropbox_rs;

use std::env;

use dropbox_rs::Dropbox;
use dropbox_rs::paper::{ListPaperDocsArgs, ListPaperDocsContinueArgs, ListPaperDocsSortBy};

#[test]
fn test_paper_list_and_continue() {
    let access_code = env::var("DROPBOX_ACCESS").expect("Couldn't find DROPBOX_ACCESS env_var");

    let api = Dropbox::new(&access_code);
    let list = api.paper_ops
        .list(&ListPaperDocsArgs {
            filter_by: None,
            sort_by: Some(ListPaperDocsSortBy::modified),
            sort_order: None,
            limit: 100,
        })
        .expect("error fetching list");

    let list_continue = api.paper_ops
        .list_continue(&ListPaperDocsContinueArgs { cursor: list.body.cursor.value })
        .expect("error fetching list/continue");
}
