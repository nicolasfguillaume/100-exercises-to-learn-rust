// TODO: Set `move_forward` to `true` in `ready` when you think you're done with this exercise.
//  Feel free to call an instructor to verify your solution!
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    // Start the system by spawning the server thread.
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    // A `Sender` instance can be used by one or more clients to interact with the server..
    sender
        // You call send on the sender to push data into the channel.
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // If the thread is no longer running, this will panic
        // because the channel will be closed.
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // There's very little that we can check automatically in this exercise,
    // since our server doesn't expose any **read** actions.
    // We have no way to know if the inserts are actually happening and if they
    // are happening correctly.
    let move_forward = false;

    assert!(move_forward);
}
