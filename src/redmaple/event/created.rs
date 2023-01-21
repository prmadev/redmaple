use crate::redmaple::id::ID;

/// Creates a new instance of Story
///
/// * `id`: is of type ID.
/// * `redmaple_id`: is of type ID.
#[derive(Clone, Debug, Default)]
pub struct Created {
    id: ID,
    redmaple_id: ID,
}

impl Created {
    pub fn new(redmaple_id: ID) -> Self {
        Self {
            id: ID::new(),
            redmaple_id,
        }
    }

    pub const fn id(&self) -> &ID {
        &self.id
    }

    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_make_event() {
        let red_maple_id = ID::new();
        let new_event = Created::new(red_maple_id.clone());

        assert_eq!(new_event.redmaple_id(), &red_maple_id);
        assert_eq!(
            new_event.id().uuid().to_string().len(),
            red_maple_id.uuid().to_string().len()
        );
    }
}
