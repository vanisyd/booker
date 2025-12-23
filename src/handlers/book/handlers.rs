use actix_web::web;
use sqlx::PgPool;
use crate::handlers::book::CreateBookRequest;
use crate::db::models::book::{Book, CreateBook};
use crate::services::open_library::{open_library, OpenLibrarySearchRequest};

pub async fn add_book( // find?
    web::Form(form): web::Form<CreateBookRequest>,
    db_pool: web::Data<PgPool>
) {
    let db_books = Book::find_by_title(&db_pool, &form.title).await.unwrap();
    let api_books = open_library::search(OpenLibrarySearchRequest {
        query: None,
        author: None,
        title: Some(form.title.clone())
    }).await;
    let books: Vec<CreateBook> = api_books.doc.iter().filter(|&book| {
        db_books.iter().find(|&db_book| db_book.title == book.title).is_none()
    }).map(|book| CreateBook {
        title: book.title.clone(),
        cover_url: None,
        published_year: None,
        page_count: None,
        external_keys: None,
    }).collect();
}

pub async fn book_list(
    db_pool: web::Data<PgPool>
) {
    
}