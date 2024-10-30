use reqwest;
use tokio;

use ticket_fields::test_helpers::{ticket_description, ticket_title};

use outro_08::store::TicketId;
use outro_08::data::{Ticket, TicketDraft, TicketPatch, Status};
use outro_08::server::start_server;

const LOCALHOST: &str = "127.0.0.1:3000";

async fn test_add_ticket() {
    let client = reqwest::Client::new();
    let url = format!("http://{}{}", LOCALHOST, "/tickets");

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };

    // Send a POST request to the API
    let response = client
        .post(url)
        .json(&draft)  // Serialize and attach the JSON payload
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        // Deserialize the response body
        let posted_ticket_id: TicketId = response.json().await.unwrap();
        println!("Deserialized POST response: {:#?}", posted_ticket_id);
        assert_eq!(posted_ticket_id.0, 0);
    } else {
        panic!("Failed to create a new Item.");
    }
}

async fn test_get_ticket(ticket_expected: Ticket) {    
    // Send a GET request to the API
    let url = format!("http://{}{}", LOCALHOST, "/tickets/0");
    let response = reqwest::get(url)
        .await
        .unwrap();

    // Ensure the request was successful
    if response.status().is_success() {
        // Parse the response as a vector of Item
        let ticket_resp = response.json::<Ticket>().await.unwrap();
        println!("GET response json: {:#?}", ticket_resp);
        assert_eq!(ticket_expected, ticket_resp);
    } else {
        panic!("Failed to fetch data. Status: {}", response.status());
    }
}

async fn test_patch_ticket() {
    let client = reqwest::Client::new();
    let url = format!("http://{}{}", LOCALHOST, "/tickets/patch");

    let patch = TicketPatch {
        id: TicketId(0),
        title: None,
        description: None,
        status: Some(Status::InProgress),
    };

    // Send a POST request to the API
    let response = client
        .post(url)
        .json(&patch)  // Serialize and attach the JSON payload
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        let _ = response.text().await.unwrap();  // Read the response body
    } else {
        panic!("Failed to patch the ticket. Status: {}", response.status());
    }
}

#[tokio::test]
async fn test_integration() {
    start_server(LOCALHOST).await;

    test_add_ticket().await;

    let ticket_expected = Ticket {
        id: TicketId(0),
        title: ticket_title(),
        description: ticket_description(),
        status: Status::ToDo,
    };

    test_get_ticket(ticket_expected).await;

    test_patch_ticket().await;

    let ticket_expected = Ticket {
        id: TicketId(0),
        title: ticket_title(),
        description: ticket_description(),
        status: Status::InProgress,
    };

    test_get_ticket(ticket_expected).await;
}
