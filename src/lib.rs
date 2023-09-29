pub mod cal;

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
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
    pub fn update() -> () {
        let mut cal = Calendar::new();

        //create an event
        let scheduled = cal.upsert(Slot {
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            end_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            ..Slot::default()
        }, Event{
            status: Attending,
            description: Some(String::from("Test Event")),
            title: Some(String::from("Test title"))
        });

        assert!(scheduled.is_some());
        assert!(scheduled.as_ref().unwrap().id.is_some());

        let tentative = cal.upsert(scheduled.unwrap(), Event {
            status: Tentative,
            description: Some(String::from("Test Event is tentative")),
            title: Some(String::from("Test title - tentative"))
        });

        assert!(tentative.is_some());
        assert!(tentative.unwrap().id.is_some());
    }
}
