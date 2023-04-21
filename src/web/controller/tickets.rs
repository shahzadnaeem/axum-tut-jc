use crate::model::tickets::{Ticket, TicketCreate};
use crate::{Error, Result};

use super::AppState;

impl AppState {
    pub async fn create_ticket(self: &Self, data: TicketCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = Ticket {
            id: store.len() as u64,
            title: data.title,
            done: false,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn get_tickets(self: &Self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(self: &Self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        // NOTE: Use idiomatic version below
        // if let Some(ticket) = ticket {
        //     Ok(ticket)
        // } else {
        //     Err(Error::DeleteTicketNotFound { id })
        // }

        ticket.ok_or(Error::DeleteTicketNotFound { id })
    }
}