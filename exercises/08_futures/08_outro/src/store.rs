use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc,RwLock};

use crate::data::{Status,Ticket,TicketDraft,TicketPatch};

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub struct TicketId(pub u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        // each ticket is protected by its own lock 
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }

    pub fn get_mut(&mut self, patch: TicketPatch) -> () {
        // get a mutable ticket, using the ticket id of the patch
        let ticket_mut: &mut Arc<RwLock<Ticket>> = self.tickets.get_mut(&patch.id).expect("Ticket not found");

        let mut ticket = ticket_mut.write().unwrap(); // Acquire a write lock to modify the Ticket

        // Only update fields that are `Some` in the patch
        if let Some(new_title) = patch.title {
            ticket.title = new_title;
        }
        if let Some(new_description) = patch.description {
            ticket.description = new_description;
        }
        if let Some(new_status) = patch.status {
            ticket.status = new_status;
        };
    }
}
