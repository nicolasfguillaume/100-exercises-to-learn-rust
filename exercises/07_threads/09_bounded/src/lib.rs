// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender, SendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, SendError<Command>> {
        // todo!()
        // Create a one-shot channel to get the result from the server
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(0);
        // build the command
        let command = Command::Insert {
            draft: draft,
            response_channel: response_sender,
        };
        // send it to the server
        self.sender
            // try_send has two failure cases instead of one (one for disconnection, one for a full buffer)
            .try_send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");
        // call recv on the response channel to get the response
        let ticket_id: TicketId = response_receiver
            .recv()
            .expect("No response received!");
        Ok(ticket_id)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, SendError<Command>> {
        // todo!()
        // Create a one-shot channel to get the result from the server
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(0);
        // build the command 
        let command = Command::Get {
            id: id,
            response_channel: response_sender,
        };
        // send it to the server
        self.sender
            .try_send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");
        // call recv on the response channel to get the response.
        let ticket: Option<Ticket> = response_receiver
            .recv()
            .expect("No response received!");
        Ok(ticket)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    // capacity is the buffer size (max number of message sent before the channel is full)
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
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
