
static BASE_URL: &'static str = "https://api.dropboxapi.com/2/paper/docs";

struct PaperOperations {
    access_token: String,
}

impl PaperOperations {
    //pub fn list(&self, request: &ListPaperDocsRequest) -> Result<ListPaperDocsResponse, Error> {}
}

/**
 * List
 **/
#[derive(Debug,Serialize,Deserialize)]
enum ListPaperDocsFilterBy {
    docs_accessed,
    docs_created,
}

#[derive(Debug,Serialize,Deserialize)]
enum ListPaperDocsSortBy {
    accessed,
    modified,
    created,
}

#[derive(Debug,Serialize,Deserialize)]
enum ListPaperDocsSortOrder {
    ascending,
    descending,
}

struct ListPaperDocsRequest {
    filter_by: Option<ListPaperDocsFilterBy>,
    sort_by: Option<ListPaperDocsSortBy>,
    sort_order: Option<ListPaperDocsSortOrder>,
    limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListPaperDocsResponse {
    doc_ids: Vec<String>,
    cursor: Cursor,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cursor {
    value: String,
    expiration: String,
}
