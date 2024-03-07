use axum::{routing::get, Router};
use routes::{crawl_scrape_me, scrape_me, scrape_me_too};

pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(scrape_me))
        .route("/scrapme", get(scrape_me_too()))
        .route("/crawl", get(crawl_scrape_me));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
