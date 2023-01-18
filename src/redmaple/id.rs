use std::fmt::Debug;

use uuid::Uuid;

/// The Implementation of the ID that the crate uses
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ID(Uuid);

impl ID {
    /// creats a new instance of the ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
