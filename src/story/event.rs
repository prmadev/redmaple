use uuid::Uuid;

use super::{
    content::{Content, ContentMode, ExistingContentID},
    id::ID,
    ExistingStoryID,
};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░ Event
*/

pub enum Event {
    Created(Created),
    ContentAdded(ContentAdded),
    ContentPublished(ContentPublished),
    ContentModed(ContentModed),
    ContentDeleted,
    EventDeleted,
}

/// Creates a new instance of Story
pub struct Created {
    id: ID,
    story_id: ID,
}

/// Adds Content to that story
pub struct ContentAdded {
    id: ID,
    story_id: ExistingStoryID,
    content: Content,
}

/// Sets a Content as published
pub struct ContentPublished {
    id: ID,
    story_id: ExistingStoryID,
    content: ExistingContentID,
}

/// Changes the mode of a content
pub struct ContentModed {
    id: ID,
    story_id: ExistingStoryID,
    content: ExistingContentID,
    new_mod: ContentMode,
}

pub struct ContentDeleted {
    id: ID,
    story_id: ExistingStoryID,
    content: ExistingContentID,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░ ID
*/

/// Creates an instance of an event the specified ID
pub struct ExistingEventID(ID);

impl ExistingEventID {
    pub fn build(id: ID) -> Self {
        todo!();
        // ExistingEventID(id)
    }
}
