use crate::db::models::{Item, ItemForm, NewItem};
use crate::db::Connection as DbConnection;

use actix_web::web::{HttpResponse, Json, Path};
use actix_web::Result;

pub fn index(conn: DbConnection) -> Result<Json<Vec<Item>>> {
    Item::select(&conn).map(|items| Json(items)).map_err(|e| {
        HttpResponse::InternalServerError()
            .body(e.to_string())
            .into()
    })
}

pub fn create(conn: DbConnection, item: Json<NewItem>) -> Result<Json<Item>> {
    item.into_inner()
        .insert(&conn)
        .map(|x| Json(x))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

pub fn show(item_id: Path<i32>, conn: DbConnection) -> Result<Json<Item>> {
    Item::find(item_id.into_inner(), &conn)
        .map(|item| Json(item))
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().body(e.to_string()),
            }
            .into()
        })
}

pub fn destroy(item_id: Path<i32>, conn: DbConnection) -> Result<()> {
    Item::destroy(item_id.into_inner(), &conn)
        .map(|_| ())
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().body(e.to_string()),
            }
            .into()
        })
}

pub fn update(item_id: Path<i32>, conn: DbConnection, item: Json<ItemForm>) -> Result<Json<Item>> {
    Item::update(item_id.into_inner(), item.into_inner(), &conn)
        .map(|item| Json(item))
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().body(e.to_string()),
            }
            .into()
        })
}
