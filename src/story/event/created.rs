use crate::story::id::ID;

/// Creates a new instance of Story
#[derive(Debug, Clone)]
pub struct Created {
    id: ID,
    story_id: ID,
}

impl Created {
    pub fn new(story_id: ID) -> Self {
        Self {
            id: ID::new(),
            story_id,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn story_id(&self) -> &ID {
        &self.story_id
    }
}
