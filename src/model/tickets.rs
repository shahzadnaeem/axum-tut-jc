use crate::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub created_by_uid: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct TicketCreate {
    pub title: String,
}

pub type TicketsStoreType = Vec<Option<Ticket>>;

#[derive(Clone)]
pub struct TicketsStore {
    pub store: Arc<Mutex<TicketsStoreType>>,
}

impl TicketsStore {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store: Arc::default(), // == new(Mutex::new(Vec::new())),
        })
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, TicketsStoreType>> {
        self.store.lock()
    }
}
