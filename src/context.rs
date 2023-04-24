#[derive(Clone, Debug)]
pub struct Context {
    user_id: i64, // NOTE: Private :)
}

// Constructor
impl Context {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

// Accessors
impl Context {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
