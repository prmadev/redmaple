mod content_added;
mod content_deleted;
mod content_moded;
mod content_published;
pub mod created;
use crate::{
    store::{ApplyError, EventStorage, StateStorage},
    view_mode::{BlogMode, ViewMode},
};

use self::created::Created;

use super::id::ID;
use std::fmt::Debug;

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone)]
pub enum Event {
    /// States that a RedMaple is created
    Created(created::Created),
    /// When a content is added. It does not neccessarily means that it is published
    ContentAdded(content_added::ContentAdded),
    /// Happens When a content is visible by all those that can view the RedMaple
    ContentPublished(content_published::ContentPublished),
    /// Happens when the view mod of the post changes
    ContentModed(content_moded::ContentModed),
    /// Happens when the content is no longer visible
    ContentDeleted(content_deleted::ContentDeleted),
}

impl Event {
    /// this is is just for the sake of ergonomics to use instead of store.apply(event)
    ///
    /// # Errors
    ///
    /// This function should return errors that means for some reasons state could not be changed
    pub fn apply(&self, store: &dyn StateStorage) -> Result<(), ApplyError> {
        store.apply(self)
    }

    /// returns the a reference to the inner ID of the event
    #[must_use]
    pub const fn id(&self) -> &ID {
        match *self {
            Self::Created(ref e) => e.id(),
            Self::ContentAdded(ref e) => e.id(),
            Self::ContentPublished(ref e) => e.id(),
            Self::ContentModed(ref e) => e.id(),
            Self::ContentDeleted(ref e) => e.id(),
        }
    }

    /// creates a new create event that starts a new redmaple
    #[must_use]
    pub fn new_create_event() -> Self {
        Self::Created(Created::new(ViewMode::Blog(BlogMode::Text), ID::new()))
    }
}

/// Creates an instance of an event the specified ID
#[derive(Clone, Debug)]
pub struct ExistingEventID {
    id: ID,
}

impl ExistingEventID {
    /// This function is a builder function that creates an instance of validated ID
    /// Which means that the ID exists in the store provided.
    /// The lifetime of the this validated type should at least be as long as the store exists
    ///
    /// * `id`: ID
    /// * `store`: generic over `ContentDataBase` which lives more than the store
    ///
    /// # Errors
    ///
    /// This function should return errors that the given ID could not be found
    pub fn build(id: ID, store: &dyn EventStorage) -> Result<Self, IDError> {
        if store.id_exists(&id) {
            Ok(Self { id })
        } else {
            Err(IDError::NotFound)
        }
    }

    /// Gets the ID inside the `ExistingEventID`
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}

/// Errors that relate to validating an ID's existence
#[derive(Debug, thiserror::Error, Clone)]
pub enum IDError {
    /// Error that means that the ID could not be associated with any entity
    #[error("Could Not be found")]
    NotFound,
}
