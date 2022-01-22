use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{self, models};

#[get("/")]
pub async fn get_items() -> Result<impl Responder> {
    let items = db::get_items();

    Ok(web::Json(items))
}

#[get("/{id}")]
pub async fn get_item(path: web::Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    let item = db::get_item(id);

    Ok(web::Json(item))
}

#[derive(Serialize)]
pub struct DeletedItem {
    affected_rows: usize,
}

#[delete("/{id}")]
pub async fn delete_item(path: web::Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    let affected_rows = db::delete_item(id);

    Ok(web::Json(DeletedItem { affected_rows }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    description: String,
}

#[post("/")]
pub async fn create_item(item: web::Json<CreateItem>) -> Result<impl Responder> {
    let new_item = models::NewItem {
        description: &item.0.description,
        id: &Uuid::new_v4().to_string(),
        done: &false,
    };
    let _created_item = db::create_item(new_item);

    Ok(item)
}
