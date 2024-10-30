use axum;
use tokio;
use hyper;

use crate::api::{root,list_items,add_item,get_item};

pub async fn start_server(url: &str) -> tokio::task::JoinHandle<Result<(), hyper::Error>> {
    // Define your routes
    let app = create_app();

    // Create a server listening on localhost:3000
    let addr = url.parse().unwrap();
    println!("Server running at http://{}", addr);
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    println!("Memory address of server: {:p}", &server);
   
    // tokio will continue to run the spawned task, in the background, concurrently with the task that spawned it
    let server_handle = tokio::task::spawn(server);
    println!("Memory address of server_handle: {:p}", &server_handle);

    // Wait for the server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    server_handle
}

fn create_app() -> axum::Router {
    // Build the application with routes
    axum::Router::new()
        // GET /
        .route("/", axum::routing::get(root))                          
        // GET /items and POST /items
        .route("/items", axum::routing::get(list_items).post(add_item))  
        // GET /items/:id
        .route("/items/:id", axum::routing::get(get_item))                              
}