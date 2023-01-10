use uuid::Uuid;

use super::id::ID;

pub struct ExistingContentID {
    id: ID,
}

pub enum ContentMode {
    HeadPost,
    Conversation,
    Edition,
}

pub enum Content {
    Text,
    Picture,
}
