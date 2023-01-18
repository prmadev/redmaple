use crate::redmaple::{content::ExistingContentID, id::ID, ExistingRedMapleID};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct ContentDeleted {
    id: ID,
    redmaple_id: ExistingRedMapleID,
    content: ExistingContentID,
}

impl ContentDeleted {
    /// Creates an event that states that some content has been deleted (invisible) to users.
    pub fn new(redmaple_id: ExistingRedMapleID, content: ExistingContentID) -> Self {
        Self {
            id: ID::new(),
            redmaple_id,
            content,
        }
    }

    /// Gets the ID of the entity
    pub fn id(&self) -> &ID {
        &self.id
    }

    /// Gets the ID of the redmaple that holds this event
    pub fn redmaple_id(&self) -> &ExistingRedMapleID {
        &self.redmaple_id
    }

    /// Gets the inner content ID that this event is effecting on
    pub fn content(&self) -> &ExistingContentID {
        &self.content
    }
}
