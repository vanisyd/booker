use crate::services::open_library::{OpenLibrarySearchRequest, OpenLibrarySearchResponse};

const RESPONSE_FIELDS: [&str; 4] = [
    "title", "key", "author_key", "author_name"
];

pub async fn search(request: OpenLibrarySearchRequest) -> OpenLibrarySearchResponse {
    let query_string = serde_urlencoded::to_string(request)
        .expect("Failed to encode OL query");
    let fields = serde_urlencoded::to_string(RESPONSE_FIELDS)
        .expect("Failed to encode response fields");
    let url = format!(
        "https://openlibrary.org/search.json?{}&fields={}",
        query_string,
        fields
    );

    let body: OpenLibrarySearchResponse = reqwest::get(url).await
        .expect("Failed to request")
        .json().await.expect("Failed to get json from request");

    body
}