mod content;
mod event;
mod id;

use uuid::Uuid;

pub struct Story {
    id: Uuid,
    view_mode: ViewMode,
    events: Vec<event::Event>,
}

pub struct ExistingStoryID {
    id: Uuid,
}

pub enum ViewMode {
    Blog(BlogMode),
    Conversation,
    ResponseLinks,
    CuratedList,
    TodoList,
}

pub enum BlogMode {
    Text,
    PhotoSlide,
    Video,
}
