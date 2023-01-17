use std::fmt::Debug;

use uuid::Uuid;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ID(Uuid);
impl ID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
