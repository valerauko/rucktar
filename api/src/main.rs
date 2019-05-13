#[macro_use]
extern crate diesel;
extern crate env_logger;

use actix_web::web;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use std::env;
use std::io;

mod db;
mod handlers;

fn main() -> io::Result<()> {
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(db_url).expect("Couldn't establish connection to database!");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
            .wrap(Logger::new("%s %Dms \"%r\" %a \"%{User-Agent}i\""))
    })
    .bind("0.0.0.0:3014")?
    .run()
}
