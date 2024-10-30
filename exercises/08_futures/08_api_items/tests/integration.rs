use reqwest;
use tokio;

use api_items::data::Item;
use api_items::server::start_server;

// Add a Rust debugger to VSCode:
// https://stackoverflow.com/questions/37586216/step-by-step-interactive-debugger-for-rust

// Gotchas:
// Tests are run in parallel by default. To run them sequentially, use the following command:
// cargo test -- --test-threads 1 --nocapture
// Tests are run in alphabetical order by function name.
// Memory address of server and the server_handle are the same across all tests.
// Which leads to think that the server is instanciated only once and shared across all tests. This needs to be confirmed.

const LOCALHOST: &str = "127.0.0.1:3000";

#[tokio::test]
async fn test_root() {
    let _ = start_server(LOCALHOST).await;

    // Send a GET request to the API
    let url = format!("http://{}{}", LOCALHOST, "/");
    let response = reqwest::get(url).await.unwrap();

    let mut body: String = Default::default();

    // Ensure the request was successful
    if response.status().is_success() {
        // Parse the response body as text
        body = response.text().await.unwrap();
        println!("GET response body: {}", body);        
    } else {
        println!("Failed to fetch data. Status: {}", response.status());
    }

    assert_eq!(body, "success");
}

#[tokio::test]
async fn test_add_item() {
    let _ = start_server(LOCALHOST).await;

    let client = reqwest::Client::new();
    let url = format!("http://{}{}", LOCALHOST, "/items");

    let item = Item {
        id: 1,
        name: "foo".to_string(),
    };

    // Send a POST request to the API
    let response = client
        .post(url)
        .json(&item)  // Serialize and attach the JSON payload
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        let posted_item: Item = response.json().await.unwrap();  // Deserialize the response body
        println!("Deserialized POST response: {:#?}", posted_item);
    } else {
        println!("Failed to create a new Item.");
    }
}

#[tokio::test]
async fn test_list_items() {
    let _ = start_server(LOCALHOST).await;

    // Send a GET request to the API
    let url = format!("http://{}{}", LOCALHOST, "/items");
    let response = reqwest::get(url)
        .await
        .unwrap();

    // Ensure the request was successful
    if response.status().is_success() {
        // Parse the response as a vector of Item
        let items: Vec<Item> = response.json().await.unwrap();
        println!("GET response json: {:#?}", items);

        let item = Item {
            id: 1,
            name: "foo".to_string(),
        };

        assert_eq!(item, items[0]);
    } else {
        println!("Failed to fetch data. Status: {}", response.status());
    }
}

#[tokio::test]
async fn test_get_item() {
    let _ = start_server(LOCALHOST).await;
    
    // Send a GET request to the API
    let url = format!("http://{}{}", LOCALHOST, "/items/1");
    let response = reqwest::get(url)
        .await
        .unwrap();

    // Ensure the request was successful
    if response.status().is_success() {
        // Parse the response as a vector of Item
        let item_resp = response.json::<Item>().await.unwrap();
        println!("GET response json: {:#?}", item_resp);

        let item = Item {
            id: 1,
            name: "foo".to_string(),
        };

        assert_eq!(item, item_resp);
    } else {
        println!("Failed to fetch data. Status: {}", response.status());
    }
}
