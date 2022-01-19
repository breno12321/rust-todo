use actix_web::{web, Responder, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{self, get_items, models};

#[get("/")]
pub async fn get_item() -> Result<impl Responder> {
    let items = get_items();

    Ok(web::Json(items))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateItem {
    description: String,
}

#[post("/")]
pub async fn create_item(item: web::Json<CreateItem>) -> Result<impl Responder> {
    let newItem = models::NewItem {
        description: &item.0.description,
        id: &Uuid::new_v4().to_string(),
        done: &false,
    };
    let items = db::create_item(newItem);

    Ok(item)
}
