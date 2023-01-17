mod content;
pub mod event;
pub mod id;
use self::{event::Event, id::ID};
use crate::store::EventStore;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Story {
    id: ID,
    view_mode: ViewMode,
    events: Vec<event::Event>,
}

impl Story {
    pub fn view_mode(&self) -> &ViewMode {
        &self.view_mode
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn events(&self) -> &[Event] {
        self.events.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct ExistingStoryID {
    id: ID,
}

#[derive(Clone, Debug)]
pub enum ViewMode {
    Blog(BlogMode),
    Conversation,
    ResponseLinks,
    CuratedList,
    TodoList,
}

#[derive(Clone, Debug)]
pub enum BlogMode {
    Text,
    PhotoSlide,
    Video,
}

impl ExistingStoryID {
    /// This function is a builder function that creates an instance of validated ID
    /// Which means that the ID exists in the store provided.
    /// The lifetime of the this validated type should at least be as long as the store exists
    ///
    /// * `id`: ID
    /// * `store`: generic over ContentDataBase which lives more than the store
    pub fn build(id: ID, store: Box<dyn EventStore>) -> Result<Self, IDError> {
        match store.id_exists(&id) {
            true => Ok(Self { id }),
            false => Err(IDError::NotFound),
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IDError {
    #[error("Could Not be found")]
    NotFound,
}
