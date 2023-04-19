//! A simple Model Layer
//! with a simple store implementation

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct TicketCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets: Arc::default(), // new(Mutex::new(Vec::new())),
        })
    }
}

// CRUD functionality

impl ModelController {
    pub async fn create_ticket(self: &Self, data: TicketCreate) -> Result<Ticket> {
        let mut store = self.tickets.lock().unwrap();

        let ticket = Ticket {
            id: store.len() as u64,
            title: data.title,
            done: false,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn get_tickets(self: &Self) -> Result<Vec<Ticket>> {
        let store = self.tickets.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(self: &Self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets.lock().unwrap();

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
