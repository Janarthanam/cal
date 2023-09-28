use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::time::Duration;
use rand::random;

#[derive(Debug,Default)]
pub enum Status {
    Attending,
    #[default]
    NotAttending,
    Tentative
}

//Slot is a basic calendar slot.
#[derive(Debug, Default, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Slot {
    pub start_time: Duration,
    pub end_time: Duration,
    //todo. this can be private
    pub id: Option<u8>,
}

#[derive(Debug,Default)]
pub struct Event {
    pub status: Status,
    pub description: Option<String>,
    pub title: Option<String>
}

#[derive(Debug)]
pub struct Calendar {
    slots: BTreeMap<Slot,Event>
}

impl Calendar {

    //initiate slots
    pub const fn new() -> Self {
        Calendar {
            slots : BTreeMap::new()
        }
    }

    //return the slots for this event.
    //when id is present only specific event is returned.
    pub fn get_events(&self, start_slot: &Slot, end_slot: &Slot) -> Vec<(&Slot,&Event)>
    {
        let events = self.slots.range((Included(start_slot), Included(end_slot)));
        events.collect()
    }


    //with this slot
    pub fn upsert(&mut self, slot: Slot, event: Event) -> () {
        if let None = slot.id  {
            self.slots.insert(Slot {
                id: Some(random()),
                ..slot
            }, event)
        } else {
            //insert event in to the sorted btree
            self.slots.insert(slot, event)
        };
    }
}

