use std::sync::{Mutex, Arc};

use actix_web::{App, HttpServer, web::{self, Data}};
use controllers::{get_all_reviews, create_review, get_review_by_id, modify_review, delete_review, get_with_stars_filter};
use db::MyDatabase;

pub mod models;
pub mod controllers;
pub mod db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Creation of our basic db (vector of Reviews).
    // Using an Arc and mutex to be thread safe.
    let db = Arc::new(Mutex::new(MyDatabase::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(Arc::clone(&db)))
        .service(
                // Our basic routing:
                web::scope("/api/books/")
                .service(get_all_reviews)
                .service(create_review)
                .service(get_review_by_id)
                .service(modify_review)
                .service(delete_review)
                .service(get_with_stars_filter)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
