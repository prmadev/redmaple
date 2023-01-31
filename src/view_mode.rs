//! different views mods for each redmaple

/// `ViewMode` trait offers a generic way of handling different view of the data
pub trait ViewMode {
    /// returns a string with the name of the `ViewMode`
    fn name(&self) -> String;
}
