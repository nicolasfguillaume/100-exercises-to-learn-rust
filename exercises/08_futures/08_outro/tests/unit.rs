use std::sync::{Arc, RwLock};
use std::thread::spawn;

use ticket_fields::test_helpers::{ticket_description, ticket_title};
use outro_08::data::{TicketDraft,TicketPatch,Status};
use outro_08::store::{TicketStore,TicketId};

// Unit tests should be run in multi thread:
// cargo test --test unit -- --nocapture 

// Why Both Ticket and TicketStore Use Arc<RwLock<T>>
// In a TicketStore that holds many Ticket instances, both the store and individual tickets are shared resources that may need 
// to be accessed and modified concurrently. By wrapping both Ticket and TicketStore in Arc<RwLock<T>>, we ensure:
// - TicketStore: The TicketStore itself can be accessed or modified safely across threads, allowing new tickets to be added 
//   or existing ones to be removed.
// - Ticket: Each Ticket within the store can also be accessed or modified individually without needing to lock the entire
//   TicketStore, making it more granular and efficient.

#[test]
fn test_multithread() {
    // Sharing TicketStore across threads
    // If we use a RwLock, we can read tickets in parallel
    // Rwlock introduce (another) lock to synchronize access to the TicketStore itself
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let store1 = store.clone();
    let client1 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        // write returns a guard that allows to modify the data
        store1
            .write()
            .unwrap()
            .add_ticket(draft)
    });

    let store2 = store.clone();
    let client2 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        // write returns a guard that allows to modify the data
        store2
            .write()
            .unwrap()
            .add_ticket(draft)
    });

    let ticket_id1 = client1.join().unwrap();
    let ticket_id2 = client2.join().unwrap();

    // read returns a guard that allows to read the data
    let reader = store.read().unwrap();

    let ticket1 = reader.get(ticket_id1).unwrap();
    assert_eq!(ticket_id1, ticket1.read().unwrap().id);

    let ticket2 = reader.get(ticket_id2).unwrap();
    assert_eq!(ticket_id2, ticket2.read().unwrap().id);
}

#[test]
fn test_add_ticket() {
    // Sharing TicketStore across threads
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let client = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        // write returns a guard that allows to modify the data
        store
            .write()
            .unwrap()
            .add_ticket(draft)
    });

    let ticket_id = client.join().unwrap();
    assert_eq!(ticket_id, TicketId(0));
}

#[test]
fn test_get_ticket() {
    // Sharing TicketStore across threads
    // If we use a RwLock, we can read tickets in parallel
    // Rwlock introduce (another) lock to synchronize access to the TicketStore itself
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let store_cloned = store.clone();
    let client = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        // write returns a guard that allows to modify the data
        store_cloned
            .write()
            .unwrap()
            .add_ticket(draft)
    });

    let ticket_id = client.join().unwrap();

    // read returns a guard that allows to read the data
    let reader = store.read().unwrap();

    let ticket = reader.get(ticket_id).unwrap();
    assert_eq!(ticket_id, ticket.read().unwrap().id);
}

#[test]
fn test_patch_ticket() {
    // Sharing TicketStore across threads with Arc
    // Synchronizing access to the store with RwLock
    // Read-heavy workload (mostly reading tickets, not modifying them), so RwLock<T> is a good choice
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let store1 = store.clone();
    let client1 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };

        store1
            // write returns a guard that allows to modify the data
            .write()
            .unwrap()
            .add_ticket(draft)
    });

    let ticket_id = client1.join().unwrap();

    let reader = store.read().unwrap();
    let ticket = reader.get(ticket_id).unwrap();

    // The write() lock in the code below will be blocked until all readers release their locks
    drop(reader); // Release the lock explicitly

    let store2 = store.clone();
    let client2 = spawn(move || {
        let patch = TicketPatch {
            id: ticket_id,
            title: None,
            description: None,
            status: Some(Status::InProgress),
        };
        
        store2  
            // write returns a guard that allows to modify the data
            .write()
            .unwrap()
            .get_mut(patch)
    });

    let _ = client2.join().unwrap();

    let reader = store.read().unwrap();
    let ticket_patched = reader.get(ticket_id).unwrap();

    assert_eq!(ticket_id, ticket_patched.read().unwrap().id);
    assert_eq!(ticket.read().unwrap().id, ticket_patched.read().unwrap().id);
    assert_eq!(ticket_patched.read().unwrap().status, Status::InProgress);
}
