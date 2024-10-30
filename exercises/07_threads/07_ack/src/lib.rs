use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { draft: data::TicketDraft, response_sender: Sender<store::TicketId> },
    Get { id: store::TicketId, response_sender: Sender<Option<data::Ticket>> }
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert { draft: ticket_draft, response_sender }) => {
                // todo!()
                let id = store.add_ticket(ticket_draft);
                println!("Server processed request: added ticket id {:?}", id);
                // The server can use this channel to send a response back to the client
                response_sender
                    .send(id)
                    .expect("The channel is closed!");
            }
            Ok(Command::Get { id, response_sender }) => {
                // todo!()
                let ticket = store.get(id);
                println!("Server processed request: got ticket id {:?}", id);
                // The server can use this channel to send a response back to the client
                response_sender
                    // We use .cloned() to convert it into an Option<Ticket>.
                    // This works because the Ticket type implements Clone. 
                    .send(ticket.cloned())
                    .expect("The channel is closed!");
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
