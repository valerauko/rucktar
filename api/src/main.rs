#[macro_use]
extern crate diesel;

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
            .service(handlers::items::handler(&"/api/items"))
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:3014")?
    .run()
}
