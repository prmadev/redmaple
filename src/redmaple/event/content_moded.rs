use crate::redmaple::{
    content::{ContentMode, ExistingContentID},
    id::ID,
    ExistingRedMapleID,
};

/// Changes the mode of a content
#[derive(Debug, Clone)]
pub struct ContentModed {
    id: ID,
    redmaple_id: ExistingRedMapleID,
    content: ExistingContentID,
    new_mod: ContentMode,
}

impl ContentModed {
    /// Creates an event that states that some content has changed their mod to a given one.
    pub fn new(
        redmaple_id: ExistingRedMapleID,
        content: ExistingContentID,
        new_mod: ContentMode,
    ) -> Self {
        Self {
            id: ID::new(),
            redmaple_id,
            content,
            new_mod,
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

    /// return the new mode that this event makes
    pub fn new_mod(&self) -> &ContentMode {
        &self.new_mod
    }
}
