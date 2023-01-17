use crate::story::{content::ExistingContentID, id::ID, ExistingStoryID};

/// Sets a Content as published
#[derive(Debug)]
pub struct ContentDeleted {
    id: ID,
    story_id: ExistingStoryID,
    content: ExistingContentID,
}

impl ContentDeleted {
    pub fn new(story_id: ExistingStoryID, content: ExistingContentID) -> Self {
        Self {
            id: ID::new(),
            story_id,
            content,
        }
    }

    pub fn story_id(&self) -> &ExistingStoryID {
        &self.story_id
    }

    pub fn content(&self) -> &ExistingContentID {
        &self.content
    }
}
