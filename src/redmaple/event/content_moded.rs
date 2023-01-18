use crate::redmaple::{
    content::{ContentMode, ExistingContentID},
    id::ID,
    ExistingStoryID,
};

/// Changes the mode of a content
#[derive(Debug, Clone)]
pub struct ContentModed {
    id: ID,
    redmaple_id: ExistingStoryID,
    content: ExistingContentID,
    new_mod: ContentMode,
}

impl ContentModed {
    pub fn new(
        redmaple_id: ExistingStoryID,
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

    pub fn redmaple_id(&self) -> &ExistingStoryID {
        &self.redmaple_id
    }

    pub fn content(&self) -> &ExistingContentID {
        &self.content
    }

    pub fn new_mod(&self) -> &ContentMode {
        &self.new_mod
    }

    pub fn id(&self) -> &ID {
        &self.id
    }
}
