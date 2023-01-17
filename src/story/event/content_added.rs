use crate::story::{content::Content, id::ID, ExistingStoryID};

/// Adds Content to that story
#[derive(Debug)]
pub struct ContentAdded {
    id: ID,
    story_id: ExistingStoryID,
    content: Content,
}

impl ContentAdded {
    pub fn new(story_id: ExistingStoryID, content: Content) -> Self {
        Self {
            id: ID::new(),
            story_id,
            content,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn story_id(&self) -> &ExistingStoryID {
        &self.story_id
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}
