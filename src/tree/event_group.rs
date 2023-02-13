//! [`EventGroup`] is the centeral way to plug-in your events and their logic
//!
//! To make event group this small I actually spent a full day working with different solutions and
//! trying different ways and pattern for implementing it
//! so enjoy

use std::time::SystemTime;

use super::id::ID;

/// [`EventGroup`] trait describes the behavior of an event.
/// Specific implementaiton is not defined here
///
/// # Example
///
/// ```
///    use redmaple::id::ID;
///    use redmaple::event_group::EventGroup;
///    use std::time::SystemTime;
///
///    struct Eg(ID, ID, std::time::SystemTime, String);
///
///    impl EventGroup for Eg {
///        fn id(&self) -> &ID {
///            &self.0
///        }
///
///        fn redmaple_id(&self) -> &ID {
///            &self.1
///        }
///
///        fn time(&self) -> &SystemTime {
///            &self.2
///        }
///
///        fn has_the_same_contents(&self, other: &Self) -> bool{
///            self.3 == other.3
///        }
///    }
///
///    fn id_works() {
///        let ev1 = Eg(ID::new(), ID::new(), SystemTime::now(), String::from(""));
///        let ev2 = Eg(ID::new(), ID::new(), SystemTime::now(), String::from(""));
///
///        // the two instances should not have the same ID
///        assert_ne!(ev1.id(), ev2.id());
///
///        // however both have the same content: `String::from("")`
///        assert!(ev1.has_the_same_contents(&ev2))
///    }
///
/// ```
pub trait EventGroup {
    /// returns the a reference to the inner ID of the event
    #[must_use]
    fn id(&self) -> &ID;

    /// returns the id of the parent redmaple
    #[must_use]
    fn redmaple_id(&self) -> &ID;

    /// returns the time of the time that that event happened at
    #[must_use]
    fn time(&self) -> &SystemTime;

    /// checks if the event have the same content of another event, but does not check for date
    /// and id which are probably unique to each event
    fn has_the_same_contents(&self, other: &Self) -> bool;
}
