use crate::db::models::{Item, NewItem, ItemForm};
use crate::db::Connection as DbConnection;

use rocket_contrib::json::{Json};
use rocket::http::{Status};
use diesel::result::{Error};

#[get("/")]
pub fn index(conn: DbConnection) -> Result<Json<Vec<Item>>, Error> {
    Ok(Json(Item::select(&conn)?))
}

#[post("/", data = "<item>")]
pub fn create(conn: DbConnection, item: Json<NewItem>) -> Result<Json<Item>, Error> {
    Ok(Json(item.into_inner().insert(&conn)?))
}

#[get("/<item_id>")]
pub fn show(item_id: i32, conn: DbConnection) -> Result<Json<Item>, Error> {
    Ok(Json(Item::find(item_id, &conn)?))
}

#[delete("/<item_id>")]
pub fn destroy(item_id: i32, conn: DbConnection) -> Result<Status, Error> {
    Item::destroy(item_id, &conn)?;

    Ok(Status::NoContent)
}

#[patch("/<item_id>", data = "<item>")]
pub fn update(item_id: i32, conn: DbConnection, item: Json<ItemForm>) -> Result<Json<Item>, Error> {
    let updated_item = Item::update(item_id, item.into_inner(), &conn)?;

    Ok(Json(updated_item))
}
