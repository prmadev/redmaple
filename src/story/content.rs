use std::fmt::Debug;

use crate::store::EventStore;

use super::id::ID;

#[derive(Clone, Debug)]
pub struct ExistingContentID {
    id: ID,
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
            true => Ok(Self { id }),
            false => Err(IDError::NotFound),
        }
    }
}

/// Content type sets the mode of each content.
#[derive(Debug, Clone)]
pub enum ContentMode {
    /// the main post of the story
    HeadPost,
    /// answer the story, or optionally, answer to another content of the same story
    Conversation(Option<ExistingContentID>),
    Edition,
}

#[derive(Debug, Clone)]
pub enum Content {
    Text,
    Picture,
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum IDError {
    #[error("Could Not be found")]
    NotFound,
}
