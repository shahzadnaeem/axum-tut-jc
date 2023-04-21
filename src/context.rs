#[derive(Clone, Debug)]
pub struct Context {
    user_id: u64, // NOTE: Private :)
}

// Constructor
impl Context {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Accessors
impl Context {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
