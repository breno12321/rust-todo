use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::schema::items;

#[derive(Serialize, Deserialize, Debug, Queryable, PartialEq)]
pub struct Item {
    pub id: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub id: &'a String,
    pub description: &'a String,
    pub done: &'a bool,
}
