use crate::story::{event::Event, id::ID, Story};

/// this trait is an adaptor trait for the storage that holds the ID stuff that this package uses in order to validate and operate
pub trait EventStore {
    fn id_exists(&self, id: &ID) -> bool;
    fn add_event(&mut self, event: Event) -> Result<(), SaveError>;
    fn get_events(&self) -> Option<Vec<Event>>;
    fn get_event(&self, id: &ID) -> Result<Event, FindError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    // #[error("Could Not be found")]
    // NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum FindError {
    #[error("Could Not be found")]
    NotFound,
}

pub trait StateStore {
    fn apply(&self, event: &Event) -> Result<(), ApplyError>;
    fn get_stories(&self) -> Option<Vec<Story>>;
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    // #[error("Could Not be found")]
    // NotFound,
}
