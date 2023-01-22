//! different views mods for each redmaple

/// Holds the different view modes that the `RedMaple` could present
#[derive(Clone, Debug)]
#[must_use]
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
