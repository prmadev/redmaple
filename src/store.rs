use crate::story::{event::Event, id::ID};

/// this trait is an adaptor trait for the storage that holds the ID stuff that this package uses in order to validate and operate
pub trait EventStore {
    fn id_exists(&self, id: &ID) -> bool;
}

pub trait StateStore {
    fn apply(&self, event: &Event) -> Result<(), ApplyError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    // #[error("Could Not be found")]
    // NotFound,
}
