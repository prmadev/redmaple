use crate::redmaple::{content::ExistingContentID, id::ID, ExistingStoryID};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct ContentDeleted {
    id: ID,
    redmaple_id: ExistingStoryID,
    content: ExistingContentID,
}

impl ContentDeleted {
    pub fn new(redmaple_id: ExistingStoryID, content: ExistingContentID) -> Self {
        Self {
            id: ID::new(),
            redmaple_id,
            content,
        }
    }

    pub fn redmaple_id(&self) -> &ExistingStoryID {
        &self.redmaple_id
    }

    pub fn content(&self) -> &ExistingContentID {
        &self.content
    }

    pub fn id(&self) -> &ID {
        &self.id
    }
}
