//! `RedMaple` offers an opinionated yet extremely flexible data modeling system based on events for backend applications
//!
//! `RedMaple` is still in its infancy. And for now, at least, it is not fully formed.
//! There is a 100 % certainty that if I can, I will strip away some items in it.
//! So please, do not use it for now. Version numbering will tell you if things got stabilised.
//!

#![deny(missing_docs)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::default_numeric_fallback,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::panic_in_result_fn,
    clippy::multiple_inherent_impl,
    clippy::map_err_ignore,
    clippy::if_then_some_else_none,
    clippy::empty_structs_with_brackets,
    clippy::useless_let_if_seq,
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::cognitive_complexity,
    clippy::self_named_constructors
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod redmaple;
pub mod store;
pub mod view_mode;

#[cfg(test)]
mod tests {

    use crate::{
        redmaple::{event::Event, id::ID},
        store::{FindError, SaveError},
    };

    use super::*;

    struct ES {
        events: Vec<Event>,
    }

    impl ES {
        fn new(events: Vec<Event>) -> Self {
            Self { events }
        }
    }

    impl store::EventStorage for ES {
        fn id_exists(&self, id: &redmaple::id::ID) -> bool {
            self.events.iter().any(|x| match x {
                Event::Created(e) => e.id() == id,
                Event::ContentAdded(e) => e.id() == id,
                Event::ContentPublished(e) => e.id() == id,
                Event::ContentModed(e) => e.id() == id,
                Event::ContentDeleted(e) => e.id() == id,
            })
        }

        fn add_event(&mut self, event: Event) -> Result<(), SaveError> {
            self.events.push(event);
            Ok(())
        }

        fn get_events(&self) -> Option<Vec<Event>> {
            let e = &self.events;
            if e.is_empty() {
                return None;
            };
            Some(e.clone())
        }

        fn get_event(&self, id: &ID) -> Result<Event, store::FindError> {
            match self.get_events() {
                Some(i) => i
                    .iter()
                    .find(|x| match x {
                        Event::Created(e) => e.id() == id,
                        Event::ContentAdded(e) => e.id() == id,
                        Event::ContentPublished(e) => e.id() == id,
                        Event::ContentModed(e) => e.id() == id,
                        Event::ContentDeleted(e) => e.id() == id,
                    })
                    .ok_or(FindError::NotFound)
                    .map(std::clone::Clone::clone),

                None => Err(FindError::NotFound),
            }
        }
    }

    #[test]
    fn it_works() {
        let _es = ES::new(vec![]);
    }
}
