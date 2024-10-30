use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        // todo!()
        // create a response channel
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        // build the command
        let command = Command::Insert {
            draft: draft,
            response_channel: response_sender,
        };
        // send it to the server
        self.sender
            .send(command)
            // If the thread is no longer running, this will panic
            // because the channel will be closed.
            .expect("Did you actually spawn a thread? The channel is closed!");
        // call recv on the response channel to get the response
        let ticket_id: TicketId = response_receiver
            .recv()
            .expect("No response received!");
        ticket_id
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        // todo!()
        // create a response channel
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        // build the command 
        let command = Command::Get {
            id: id,
            response_channel: response_sender,
        };
        // send it to the server
        self.sender
            .send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");
        // call recv on the response channel to get the response.
        let ticket: Option<Ticket> = response_receiver
            .recv()
            .expect("No response received!");
        ticket
    }
}

// Start the system by spawning the server thread.
// It returns a `TicketStoreClient` instance which can then be used
// by one client to interact with the server.
pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    // todo!()
    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
