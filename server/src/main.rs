pub mod test;

use std::fs;

use axum::{routing::get, Router};
use projcore::AsyncError;

#[tokio::main]
async fn main() -> Result<(), AsyncError> {
    simple_logger::init()?;
    log::info!("Server started!");

    let app = Router::new()
        .route("/get_book", get(get_book))
        .route("/book_info", get(book_info))
        .route("/search", get(search));

    //test::book().to_server_rsbook("books/").await?;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_book(name: String) {}

async fn book_info(name: String) -> Vec<u8> {
    log::info!("get book info: {name}");
    fs::read("books/".to_string() + &name + ".srsmeta").unwrap_or(vec![0])
}

async fn search(query: String) {}
