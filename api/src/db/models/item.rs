use crate::db::schema::items;
use crate::db::schema::items::dsl;
use crate::db::DbConnection;

use serde::{Deserialize, Serialize};

use diesel;
use diesel::prelude::*;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use chrono::offset::Utc;
use chrono::DateTime;

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub description: String,
}

#[derive(Debug, AsChangeset, Deserialize, Serialize)]
#[table_name = "items"]
pub struct ItemForm {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl NewItem {
    pub fn insert(self, conn: &DbConnection) -> QueryResult<Item> {
        use crate::db::schema::items::dsl::*;

        diesel::insert_into(items).values(&self).get_result(conn)
    }
}

impl Item {
    pub fn select(conn: &DbConnection) -> QueryResult<Vec<Item>> {
        dsl::items.order(dsl::created_at.desc()).get_results(conn)
    }

    pub fn find(item_id: i32, conn: &DbConnection) -> QueryResult<Item> {
        dsl::items.find(item_id).get_result(conn)
    }

    pub fn destroy(item_id: i32, conn: &DbConnection) -> QueryResult<usize> {
        use crate::db::schema::items::dsl::*;

        diesel::delete(items.find(item_id)).execute(conn)
    }

    pub fn update(item_id: i32, values: ItemForm, conn: &DbConnection) -> QueryResult<Item> {
        use crate::db::schema::items::dsl::*;

        diesel::update(items.find(item_id))
            .set(values)
            .get_result(conn)
    }
}
