use std::fmt::Debug;

use crate::store::EventStore;

use super::id::ID;

/// An instance of ID that is guranteed to be pointed to an existing Content
///
/// * `id`: holds an instance of ID
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

    /// Gets the ID inside the ExistingRedMapleID
    pub fn id(&self) -> &ID {
        &self.id
    }
}

/// Content type sets the mode of each content.
#[derive(Debug, Clone)]
pub enum ContentMode {
    /// The main post of the redmaple
    HeadPost,
    /// Answer the redmaple, or optionally, answer to another content of the same redmaple
    Conversation(Option<ExistingContentID>),
    /// A new edition for the headpost
    Edition,
}

/// Content holds the different forms of content
#[derive(Debug, Clone)]
pub enum Content {
    /// A Text content is a string.
    Text,
    /// A Text content is a subscription to a picture stream.
    Picture,
}

/// Errors that relate to validating an ID's existence
#[derive(Debug, thiserror::Error, Clone)]
pub enum IDError {
    /// Error that means that the ID could not be associated with any entity
    #[error("Could Not be found")]
    NotFound,
}
