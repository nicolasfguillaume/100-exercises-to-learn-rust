use axum::{Json, extract::Path, http::StatusCode};

use crate::data::Item;

// In-memory storage for items (for demonstration purposes)
static mut ITEMS_STORE: Vec<Item> = Vec::new();

// Handler for GET /
pub async fn root() -> &'static str {
    "success"
}

// Handler for GET /items - list all items
pub async fn list_items() -> Json<Vec<Item>> {
    unsafe { Json(ITEMS_STORE.clone()) }
}

// Handler for POST /items - add a new item
pub async fn add_item(Json(new_item): Json<Item>) -> (StatusCode, Json<Item>) {
    unsafe {
        ITEMS_STORE.push(new_item.clone());
    }
    (StatusCode::CREATED, Json(new_item))
}

// Handler for GET /items/:id - get item by ID
pub async fn get_item(Path(id): Path<u32>) -> Result<Json<Item>, StatusCode> {
    unsafe {
        ITEMS_STORE.iter()
            .find(|item| item.id == id)
            .cloned()
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    }
}
