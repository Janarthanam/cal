pub mod cal;

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use crate::cal::{Calendar, Event, Slot};
    use crate::cal::Status::Attending;

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
}
