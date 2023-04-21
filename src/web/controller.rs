use crate::model::tickets::TicketsStore;
use crate::Result;

pub mod tickets;

#[derive(Clone)]
pub struct AppState {
    tickets_store: TicketsStore,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: TicketsStore::new()?,
        })
    }
}
