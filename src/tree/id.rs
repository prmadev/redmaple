use std::fmt::Debug;

use uuid::Uuid;

/// The Implementation of the ID that the crate uses
///
/// # Example
///
/// ```rust
/// use redmaple::tree::id::ID;
///
/// let new_id = ID::new();
///
/// assert_eq!(4usize, new_id.into_inner().get_version_num())
///
/// ```

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ID(Uuid);

impl ID {
    /// creats a new instance of the ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn into_inner(&self) -> Uuid {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_a_new_id() {
        assert!(!ID::new().into_inner().is_nil());
    }

    #[test]
    fn check_if_two_are_similar() {
        assert_ne!(ID::new(), ID::new());
    }
}
