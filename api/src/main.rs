#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use std::env;

mod db;
mod handlers;

fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(db_url).expect("Couldn't establish connection to database!");

    rocket::ignite()
        .mount("/api/items", routes![
            handlers::items::show,
            handlers::items::index,
            handlers::items::create,
            handlers::items::update,
            handlers::items::destroy
        ])
        .manage(pool)
        .launch();
}
