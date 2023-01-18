use crate::redmaple::{content::Content, id::ID, ExistingRedMapleID};

/// Adds Content to that redmaple
#[derive(Debug, Clone)]
pub struct ContentAdded {
    id: ID,
    redmaple_id: ExistingRedMapleID,
    content: Content,
}

impl ContentAdded {
    /// Creates an event that states that some content has been added to an existing `RedMaple`.
    pub fn new(redmaple_id: ExistingRedMapleID, content: Content) -> Self {
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

    /// Gets the inner content that is represented by this event
    pub fn content(&self) -> &Content {
        &self.content
    }
}
