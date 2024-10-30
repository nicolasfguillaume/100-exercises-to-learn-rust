use axum;
use tokio;
use hyper;
use std::sync::{Arc, RwLock};

use crate::api::{add_ticket, get_ticket, patch_ticket};
use crate::store::TicketStore;

// API should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket

pub async fn start_server(url: &str) -> tokio::task::JoinHandle<Result<(), hyper::Error>> {
    // Initialize an empty TicketStore wrapped in Arc and RwLock for shared access
    let store = Arc::new(RwLock::new(TicketStore::new()));

    // Define routes
    let app = create_app()
        // Add middleware that inserts the state into all incoming request's
        // extensions. This allows the handlers to access the state.
        .layer(axum::extract::Extension(store));

    // Create a server listening on localhost:3000
    let addr = url.parse().unwrap();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
   
    // tokio will continue to run the spawned task, in the background, concurrently with the task that spawned it
    let server_handle = tokio::task::spawn(server);

    // Wait for the server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("Server running at http://{}", addr);

    server_handle
}

fn create_app() -> axum::Router {
    // Build the application with routes
    axum::Router::new()                     
        // POST /tickets
        .route("/tickets", axum::routing::post(add_ticket))
        // POST /tickets/patch
        .route("/tickets/patch", axum::routing::post(patch_ticket))  
        // GET /tickets/:id
        .route("/tickets/:id", axum::routing::get(get_ticket))                              
}