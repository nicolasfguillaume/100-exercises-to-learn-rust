use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Implement the `IntoIterator` trait for `&TicketStore` so that the test compiles and passes.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

// Implementing the `IntoIterator` trait for a reference to `TicketStore`, where 'a is the lifetime of the borrowed reference.
impl<'a> IntoIterator for &'a TicketStore {
    // The type of items being iterated over (here, a reference to each ticket), which live as long as the lifetime 'a.
    type Item = &'a Ticket;
    
    // The iterator type is a slice iterator over `&'a Ticket`, which iterates over references (&Ticket) to the elements of the Vec<Ticket>.
    type IntoIter = std::slice::Iter<'a, Ticket>;

    // The `into_iter` method returns an iterator over the tickets
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter() // Return a slice iterator that iterates over the references to the tickets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
