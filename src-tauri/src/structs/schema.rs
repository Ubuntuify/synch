use agdb::{DbId, UserValue};
use chrono::Weekday;
use serde::Serialize;

#[derive(UserValue, Debug)]
pub struct ScheduleTimeSlotEntry {
    pub db_id: Option<DbId>,
    pub slot_metadata: u64, // binary representation, first four bits (weekday), eight bits (slot)
    pub start_time: String,
    pub end_time: String,
    pub flags: u64,
}
impl ScheduleTimeSlotEntry {
    pub fn time_format(&self) -> &str {
        "%H:%M"
    }

    pub fn generate_slot_metadata(weekday: Weekday, slot: u8) -> u64 {
        (weekday.num_days_from_monday() * 8) as u64 + slot as u64
    }
}

#[derive(UserValue, Debug)]
pub struct Room {
    pub db_id: Option<DbId>,
    pub name: String,
}

#[derive(UserValue, Debug)]
pub struct Class {
    pub db_id: Option<DbId>,
    pub name: String,
}

impl Serialize for Class {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}
