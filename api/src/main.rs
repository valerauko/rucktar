#[macro_use]
extern crate diesel;

use actix_web::web;
use actix_web::{App, HttpServer};

use std::env;
use std::io;

mod db;
mod handlers;

fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = db::init_pool(db_url).expect("Couldn't establish connection to database!");
        App::new()
            .data(pool)
            .service(
                web::resource("/api/items")
                    .route(web::get().to(handlers::items::index))
                    .route(web::post().to(handlers::items::create)),
            )
            .service(
                web::resource("/api/items/{item_id}")
                    .route(web::get().to(handlers::items::show))
                    .route(web::delete().to(handlers::items::destroy))
                    .route(web::patch().to(handlers::items::update)),
            )
    })
    .bind("0.0.0.0:3014")?
    .run()
}
