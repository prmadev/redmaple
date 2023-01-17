mod content_added;
mod content_deleted;
mod content_moded;
mod content_published;
mod created;
use crate::store::{ApplyError, EventStore, StateStore};

use super::id::ID;
use std::fmt::Debug;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░ Event
*/

#[derive(Debug)]
pub enum Event {
    Created(created::Created),
    ContentAdded(content_added::ContentAdded),
    ContentPublished(content_published::ContentPublished),
    ContentModed(content_moded::ContentModed),
    ContentDeleted(content_deleted::ContentDeleted),
}

impl Event {
    /// this is is just for the sake of ergonomics to use instead of store.apply(event)
    pub fn apply(&self, store: Box<dyn StateStore>) -> Result<(), ApplyError> {
        store.apply(self)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░ ID
*/

/// Creates an instance of an event the specified ID
pub struct ExistingEventID {
    id: ID,
    // store is here to confirm that the existing content lives long enough
    _store: Box<dyn EventStore>,
}

impl Debug for ExistingEventID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExistingEventID")
            .field("id", &self.id)
            // .field("store", &self.store.)
            .finish()
    }
}

impl ExistingEventID {
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
