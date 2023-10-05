use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use cal;
use rocket::{get, launch, post, routes};
use rocket::form::{Lenient, Form};
use rocket::serde::json::Json;
use cal::cal::{Event, Slot, Calendar, SlotEvent, Calendars};

lazy_static! {
    static ref CAL:RwLock<Calendars> = RwLock::new(Calendars::new());
}


#[get("/status")]
fn hello() -> &'static str {
    "Ok"
}

#[get("/calendar/<cal>/events?<start_time>&<end_time>", format="json")]
fn get_events(cal: String, start_time: usize, end_time: usize) -> Json<Vec<(Slot,Event)>> {
    //todo: try range
    let cal_read = CAL.read().unwrap();
    let calendar = cal_read.get_instance(&cal).unwrap();

    let res = calendar.get_events(&Slot {
        start_time: start_time as u64,
        end_time: start_time as u64,
        ..<Slot as Default>::default()
    }, &Slot {
        start_time: end_time as u64,
        end_time: end_time  as u64,
        ..<Slot as Default>::default()
    }).iter().map(|it| (it.0.clone(),it.1.clone())).collect();
    Json(res)
}

#[post("/calendar/<cal>/events", format="json", data="<event>")]
fn create_event(cal: String, event: Json<SlotEvent>) -> () {
    let mut cal_write = CAL.write().unwrap();
    let calendar = cal_write.create_if_absent(&cal);

    calendar.upsert(event.slot.clone(), event.event.clone());
}

#[launch]
fn rocket() -> _ { //rust macros are crazy
    rocket::build().mount("/v1", routes![hello,get_events,create_event])
}