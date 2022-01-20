pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;

use std::env;

use models::*;

use schema::items::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_items() -> Vec<Item> {
    let connection = establish_connection();
    items
        .load::<Item>(&connection)
        .expect("Error loading posts")
}

pub fn get_item(item_id: String) -> Item {
    println!("{}", item_id);
    let connection = establish_connection();
    items
        .find(item_id)
        .get_result::<Item>(&connection)
        .expect("Error loading posts")
    // .load::<Item>(&connection)
}

pub fn create_item(new_item: NewItem) -> NewItem {
    let connection = establish_connection();

    diesel::insert_into(items)
        .values(&new_item)
        .execute(&connection)
        .expect("Error saving new item");
    new_item
}
