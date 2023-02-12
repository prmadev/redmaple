//! [`EventGroup`] is the centeral way to plug-in your events and their logic
//!
//! To make event group this small I actually spent a full day working with different solutions and
//! trying different ways and pattern for implementing it
//! so enjoy
use super::id::ID;

/// [`EventGroup`] trait describes the behavior of an event.
/// Specific implementaiton is not defined here
///
/// ```
///    use redmaple::id::ID;
///    use redmaple::event_group::EventGroup;
///
///    struct Eg(ID, ID);
///    impl EventGroup for Eg {
///        fn id(&self) -> &ID {
///            &self.0
///        }
///
///        fn redmaple_id(&self) -> &ID {
///            &self.1
///        }
///    }
///
///    fn id_works() {
///        let ev1 = Eg(ID::new(), ID::new());
///        let ev2 = Eg(ID::new(), ID::new());
///        assert_ne!(ev1.id(), ev2.id());
///    }
///
/// ```
pub trait EventGroup {
    /// returns the a reference to the inner ID of the event
    #[must_use]
    fn id(&self) -> &ID;

    /// returns the id of the parent redmaple
    fn redmaple_id(&self) -> &ID;

    // TODO: add time event
    // TODO: compare events
}
