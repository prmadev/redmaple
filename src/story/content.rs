use std::fmt::Debug;

use crate::store::EventStore;

use super::id::ID;

pub struct ExistingContentID {
    id: ID,
    // store is here to confirm that the existing content lives long enough
    _store: Box<dyn EventStore>,
}

impl Debug for ExistingContentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExistingContentID")
            .field("id", &self.id)
            // .field("store", &self.store.)
            .finish()
    }
}

impl ExistingContentID {
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

#[derive(Debug)]
/// Content type sets the mode of each content.
pub enum ContentMode {
    /// the main post of the story
    HeadPost,
    /// answer the story, or optionally, answer to another content of the same story
    Conversation(Option<ExistingContentID>),
    Edition,
}

#[derive(Debug)]
pub enum Content {
    Text,
    Picture,
}

#[derive(Debug, thiserror::Error)]
pub enum IDError {
    #[error("Could Not be found")]
    NotFound,
}
