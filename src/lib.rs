#[warn(clippy::pedantic)]
pub mod store;
#[warn(clippy::pedantic)]
pub mod story;

#[cfg(test)]
mod tests {

    use crate::{
        store::{FindError, SaveError},
        story::{event::Event, id::ID},
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

    impl store::EventStore for ES {
        fn id_exists(&self, id: &story::id::ID) -> bool {
            !self
                .events
                .iter()
                .find(|x| match x {
                    Event::Created(e) => e.id() == id,
                    Event::ContentAdded(e) => e.id() == id,
                    Event::ContentPublished(e) => e.id() == id,
                    Event::ContentModed(e) => e.id() == id,
                    Event::ContentDeleted(e) => e.id() == id,
                })
                .is_some()
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
            return Some(e.to_vec());
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
                    .map(|x| x.to_owned()),

                None => Err(FindError::NotFound),
            }
        }
    }

    #[test]
    fn it_works() {
        let _es = ES::new(vec![]);
    }
}
