mod content;
use self::{event::Event, id::ID};
use crate::store::EventStore;
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
    /// Gets the view mode of the `RedMaple`
    #[must_use] pub fn view_mode(&self) -> &ViewMode {
        &self.view_mode
    }

    /// Gets the ID of the `RedMaple`
    #[must_use] pub fn id(&self) -> &ID {
        &self.id
    }

    /// Gets an array of the events of the `RedMaple`
    #[must_use] pub fn events(&self) -> &[Event] {
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

/// Holds the different view modes that the `RedMaple` could present
#[derive(Clone, Debug)]
pub enum ViewMode {
    /// means one big post up, and editions for that post + comments and replies
    Blog(BlogMode),
    /// conversation means a series of talks that two or more people could have, responding to
    /// each other
    Conversation,
    /// a series of links that reflect a replied conversations. reflecting and responding to each
    /// other.
    ResponseLinks,
    /// a curated list of links that hold the same theme
    CuratedList,
    /// a list of todo items representing a task progress
    TodoList,
}

/// blogs could form different views, on could stress on the text while the other could present a
/// series of photos or a video
#[derive(Clone, Debug)]
pub enum BlogMode {
    /// Text is the traditional essay blogging form
    Text,
    /// PhotoSlide does not neccessarily means that the post should have an slider, rather, the
    /// focus is the photos taken.
    PhotoSlide,
    /// Video represent a video stream as the main post
    Video,
}

impl ExistingRedMapleID {
    /// This function is a builder function that creates an instance of validated ID
    /// Which means that the ID exists in the store provided.
    /// The lifetime of the this validated type should at least be as long as the store exists
    ///
    /// * `id`: ID
    /// * `store`: generic over `ContentDataBase` which lives more than the store
    pub fn build(id: ID, store: Box<dyn EventStore>) -> Result<Self, IDError> {
        match store.id_exists(&id) {
            true => Ok(Self { id }),
            false => Err(IDError::NotFound),
        }
    }

    /// Gets the ID inside the `ExistingRedMapleID`
    #[must_use] pub fn id(&self) -> &ID {
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
