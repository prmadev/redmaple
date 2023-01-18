use crate::redmaple::{content::Content, id::ID, ExistingStoryID};

/// Adds Content to that redmaple
#[derive(Debug, Clone)]
pub struct ContentAdded {
    id: ID,
    redmaple_id: ExistingStoryID,
    content: Content,
}

impl ContentAdded {
    pub fn new(redmaple_id: ExistingStoryID, content: Content) -> Self {
        Self {
            id: ID::new(),
            redmaple_id,
            content,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn redmaple_id(&self) -> &ExistingStoryID {
        &self.redmaple_id
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}
