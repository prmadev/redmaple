use crate::redmaple::id::ID;

/// Creates a new instance of Story
#[derive(Debug, Clone)]
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

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }
}
