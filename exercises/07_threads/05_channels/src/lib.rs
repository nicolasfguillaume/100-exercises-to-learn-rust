// mpsc stands for multi-producer, single-consumer
// The server will run in a separate thread (single-consumer), 
// while the clients (multiple threads) will send requests to the server.
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
    Terminate,
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut store = store::TicketStore::new();
    loop {
        // You call recv on the receiver to pull data from the channel.
        match receiver.recv() {
            Ok(message) => match message {
                Command::Insert(ticket_draft) => {
                    let id = store.add_ticket(ticket_draft);
                    println!("Server processed request: added ticket id {:?}", id);
                },
                Command::Terminate => {
                    println!("Server shutting down...");
                    break;  // Exit the loop and stop the server
                }
            }
            Err(_) => {
                println!("Server: Channel closed or no more messages.");
                break;
            }
        }
    }
}
