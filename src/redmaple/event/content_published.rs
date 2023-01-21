use crate::redmaple::{content::ExistingContentID, id::ID, ExistingRedMapleID};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct ContentPublished {
    id: ID,
    redmaple_id: ExistingRedMapleID,
    content: ExistingContentID,
}

impl ContentPublished {
    pub fn new(redmaple_id: ExistingRedMapleID, content: ExistingContentID) -> Self {
        Self {
            id: ID::new(),
            redmaple_id,
            content,
        }
    }

    pub const fn redmaple_id(&self) -> &ExistingRedMapleID {
        &self.redmaple_id
    }

    pub const fn content(&self) -> &ExistingContentID {
        &self.content
    }

    pub const fn id(&self) -> &ID {
        &self.id
    }
}
