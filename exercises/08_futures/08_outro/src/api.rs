use axum::{Json, extract::Path, http::StatusCode, Extension, response::IntoResponse};
use std::sync::{Arc, RwLock};

use crate::store::{TicketStore, TicketId};
use crate::data::{TicketDraft, TicketPatch};

// Handler for POST /tickets - add a new ticket
pub async fn add_ticket(
    Extension(store): Extension<Arc<RwLock<TicketStore>>>,
    Json(draft): Json<TicketDraft>, 
// the function returns some type that implements the specified trait (IntoResponse), without exposing the exact type
) -> impl IntoResponse {
    let ticket_id: TicketId = tokio::spawn(async move {
        store.write().unwrap().add_ticket(draft)
    })
    .await
    .expect("Created new ticket failed");

    (StatusCode::CREATED, Json(ticket_id))
}

// Handler for GET /tickets/:id - get ticket by ID
pub async fn get_ticket(
    Extension(store): Extension<Arc<RwLock<TicketStore>>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    // Acquire the read lock for the TicketStore
    let ts_reader = store.read().expect("Cannot get read lock for TicketStore");

    let ticket_id = TicketId(id);
    let ticket_locked = ts_reader.get(ticket_id).expect("Cannot find ticket in TicketStore");

    // Acquire the read lock for the Ticket
    let ticket_reader = ticket_locked.read().expect("Cannot get read lock for Ticket");
    // RwLockReadGuard only provides a read-only, temporary view
    let ticket = ticket_reader.clone();

    (StatusCode::OK, Json(ticket))
}

// Handler for POST /tickets - patch an existing ticket
pub async fn patch_ticket(
    Extension(store): Extension<Arc<RwLock<TicketStore>>>,
    Json(patch): Json<TicketPatch>,
) -> impl IntoResponse {
    let _ = tokio::spawn(async move {
        store.write().unwrap().get_mut(patch)
    })
    .await
    .expect("Patch ticket failed");

    (StatusCode::OK, ()) 
}
