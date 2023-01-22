//! redmaple is the central data-structure that is underlying the whole crate
mod content;
use self::{
    event::{created::Created, Event},
    id::ID,
};
use crate::{store::EventStorage, view_mode::ViewMode};
use std::fmt::Debug;

/// event module holds the types and functions that events could take and the operations that they
/// can do.
pub mod event;
/// id module holds the implementation of ID type
pub mod id;

/// `RedMaple` is essentially a series of related events that form a state
///
/// * `id`: of type ID
/// * `view_mode`: an enum that holds set view mode of an `RedMaple`
/// * `events`: a list of entities that happened in time series
#[derive(Debug, Clone)]
pub struct RedMaple {
    id: ID,
    view_mode: ViewMode,
    events: Vec<event::Event>,
}

impl RedMaple {
    /// creates a new instance of [`RedMaple`]
    ///
    /// * `view_mode`: sets the view mode of the `RedMaple`
    #[must_use]
    pub fn new(view_mode: ViewMode) -> Self {
        let create_event = Event::new_create_event();
        Self {
            id: create_event.id().clone(),
            view_mode,
            events: vec![create_event],
        }
    }
    /// Creates a new `RedMaple` using the given Create events
    #[must_use]
    pub fn from_create(created: &Created) -> Self {
        Self {
            id: created.redmaple_id().clone(),
            view_mode: created.view_mode().clone(),
            events: vec![Event::Created(created.clone())],
        }
    }
    /// Gets the view mode of the `RedMaple`
    pub const fn view_mode(&self) -> &ViewMode {
        &self.view_mode
    }

    /// Gets the ID of the `RedMaple`
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// Gets an array of the events of the `RedMaple`
    #[must_use]
    pub fn events(&self) -> &[Event] {
        self.events.as_ref()
    }
}

/// An instance of ID that is guranteed to be pointed to an existing `RedMaple`
///
/// * `id`: holds an instance of ID
#[derive(Clone, Debug)]
pub struct ExistingRedMapleID {
    id: ID,
}

impl ExistingRedMapleID {
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

    /// Gets the ID inside the `ExistingRedMapleID`
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}

/// Errors that relate to validating an ID's existence
#[derive(Debug, thiserror::Error)]
pub enum IDError {
    /// Error that means that the ID could not be associated with any entity
    #[error("Could Not be found")]
    NotFound,
}
