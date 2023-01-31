//! redmaple is the central data-structure that is underlying the whole crate
use self::{event_group::EventGroup, id::ID};
use crate::view_mode::ViewMode;
use std::fmt::Debug;

/// event module holds the types and functions that events could take and the operations that they
/// can do.
pub mod event_group;
/// id module holds the implementation of ID type
pub mod id;

/// `RedMaple` is essentially a series of related events that form a state
///
/// * `id`: of type ID
/// * `view_mode`: an enum that holds set view mode of an `RedMaple`
/// * `events`: a list of entities that happened in time series
#[derive(Debug, Clone)]
pub struct RedMaple<T: EventGroup + Sized + Clone, V: ViewMode + Sized + Clone> {
    id: ID,
    view_mode: V,
    events: Vec<T>,
}

impl<T: EventGroup + Sized + Clone, V: ViewMode + Sized + Clone> RedMaple<T, V> {
    /// creates a new instance of [`RedMaple`]
    ///
    /// * `view_mode`: sets the view mode of the `RedMaple`
    #[must_use]
    pub fn new(view_mode: &V, id: &ID) -> Self {
        Self {
            id: id.clone(),
            view_mode: view_mode.clone(),
            events: vec![],
        }
    }

    /// Gets the view mode of the `RedMaple`
    pub const fn view_mode(&self) -> &V {
        &self.view_mode
    }

    /// Gets the ID of the `RedMaple`
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// Gets an array of the events of the `RedMaple`
    #[must_use]
    pub const fn events(&self) -> &Vec<T> {
        &self.events
    }
}
