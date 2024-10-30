use std::sync::{Arc, RwLock};
use std::thread::spawn;

use ticket_fields::test_helpers::{ticket_description, ticket_title};
use without_channels::data::TicketDraft;
use without_channels::store::TicketStore;

#[test]
fn works() {
    // Sharing TicketStore across threads
    // If we use a RwLock, we can read tickets in parallel
    // Rwlock introduce a lock to synchronize access to the TicketStore itself
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let store1 = store.clone();
    let client1 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        // write returns a guard that allows to modify the data
        store1.write().unwrap().add_ticket(draft)
    });

    let store2 = store.clone();
    let client2 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        // write returns a guard that allows to modify the data
        store2.write().unwrap().add_ticket(draft)
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
