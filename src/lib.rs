pub mod cal;

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use std::u64::MAX;
    use crate::cal::{Calendar, Event, Slot};
    use crate::cal::Status::{Attending, Tentative};

    #[test]
    pub fn get_empty() {
        let cal = Calendar::new();
        let events = cal.get_events(&Slot{
            start_time: Duration::from_secs(0),
            end_time: Duration::from_secs(0),
            ..Slot::default()
        }, &Slot{
            start_time: Duration::from_secs(u64::MAX),
            end_time: Duration::from_secs(u64::MAX),
            ..Slot::default()
        });
        assert!(events.is_empty())
    }

    #[test]
    pub fn get_range() {
        let mut cal = Calendar::new();
        let meet_slot = Slot {
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            end_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            ..Slot::default()
        };

        let expected = meet_slot.clone();
        cal.upsert(meet_slot, Event {
            status: Attending,
            description: None,
            title: None,
        });

        let events = cal.get_events(&Slot{
            start_time: Duration::from_secs(0),
            end_time: Duration::from_secs(0),
            ..Slot::default()
        }, &Slot{
            start_time: Duration::from_secs(u64::MAX),
            end_time: Duration::from_secs(u64::MAX),
            ..Slot::default()
        });
        assert_eq!(1, events.len());
        assert_eq!(expected.start_time, events[0].0.start_time)
    }

    #[test]
    pub fn crud_test() -> () {
        let mut cal = Calendar::new();

        let slot = Slot {
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            end_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            ..Slot::default()
        };

        let mut retrieve_slot = slot.clone();

        //create an event
        let scheduled = cal.upsert(slot, Event{
            status: Attending,
            description: Some(String::from("Test Event")),
            title: Some(String::from("Test title"))
        });

        assert!(scheduled.is_some());
        assert!(scheduled.as_ref().unwrap().id.is_some());

        //retrieve
        let event = cal.get_events(&retrieve_slot, &Slot {
            start_time: Duration::from_secs(MAX),
            end_time: Duration::from_secs(MAX),
            id: None
        });

        assert!(!event.is_empty());
        assert_eq!(1, event.len());
        assert_eq!(Attending, event[0].1.status);
        assert_eq!(String::from("Test title"), event[0].1.title.clone().unwrap());

        retrieve_slot.id = event[0].0.id;

        //update
        let tentative = cal.upsert(scheduled.unwrap(), Event {
            status: Tentative,
            description: Some(String::from("Test Event is tentative")),
            title: Some(String::from("Test title - tentative"))
        });

        assert!(tentative.is_some());
        assert!(tentative.unwrap().id.is_some());

        //retrieve
        let retrieved_event = cal.get_events(&retrieve_slot, &Slot {
            start_time: Duration::from_secs(MAX),
            end_time: Duration::from_secs(MAX),
            id: None
        });

        assert!(!retrieved_event.is_empty());
        assert_eq!(1, retrieved_event.len());
        assert_eq!(Tentative, retrieved_event[0].1.status);
        assert_eq!(String::from("Test title - tentative"), retrieved_event[0].1.title.clone().unwrap());

        //delete
        let deleted_event = cal.delete(&retrieve_slot);

        assert!(deleted_event.is_some());
        assert_eq!(Tentative, deleted_event.as_ref().unwrap().status);
        assert_eq!(String::from("Test title - tentative"), deleted_event.unwrap().title.unwrap());

        //double delete
        let double_deleted = cal.delete(&retrieve_slot);
        assert!(double_deleted.is_none());

        //retrieve after delete
        let retrieved_deleted = cal.get_events(&retrieve_slot, &Slot {
            start_time: Duration::from_secs(MAX),
            end_time: Duration::from_secs(MAX),
            id: None
        });

        assert!(retrieved_deleted.is_empty());
    }
}
