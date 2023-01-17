mod content;
mod event;
mod id;
mod store;

use self::{id::ID, store::EventStore};
use std::fmt::Debug;

pub struct Story {
    id: ID,
    view_mode: ViewMode,
    events: Vec<event::Event>,
}

pub struct ExistingStoryID {
    id: ID,
    // store is here to confirm that the existing content lives long enough
    _store: Box<dyn EventStore>,
}

impl Debug for ExistingStoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExistingStoryID")
            .field("id", &self.id)
            // .field("store", &self.store.)
            .finish()
    }
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

impl ExistingStoryID {
    /// This function is a builder function that creates an instance of validated ID
    /// Which means that the ID exists in the store provided.
    /// The lifetime of the this validated type should at least be as long as the store exists
    ///
    /// * `id`: ID
    /// * `store`: generic over ContentDataBase which lives more than the store
    pub fn build(id: ID, store: Box<dyn EventStore>) -> Result<Self, IDError> {
        match store.id_exists(&id) {
            true => Ok(Self { id, _store: store }),
            false => Err(IDError::NotFound),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IDError {
    #[error("Could Not be found")]
    NotFound,
}
