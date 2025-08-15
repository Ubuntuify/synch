use agdb::{Db, QueryBuilder, QueryError, QueryId};
use chrono::{Datelike, Local, NaiveTime};
use tracing::{debug, info};

use crate::structs::{Class, ScheduleTimeSlotEntry};

#[tracing::instrument]
fn get_current_schedule(db: &Db) -> Result<Vec<ScheduleTimeSlotEntry>, QueryError> {
    info!("Starting search for current schedule time slot.");

    let mut entries = crate::db::list_time_slots(db, None)?;

    debug!(
        "Found {} schedule entries available, searching through them.",
        entries.len()
    );

    entries.retain(|t| {
        let start_time = NaiveTime::parse_from_str(&t.start_time, t.time_format()).unwrap();
        let end_time = NaiveTime::parse_from_str(&t.end_time, t.time_format()).unwrap();
        let is_time = start_time <= Local::now().time() && Local::now().time() < end_time;

        let weekday = match t.slot_metadata / 8 {
            1 => Some(chrono::Weekday::Mon),
            2 => Some(chrono::Weekday::Tue),
            3 => Some(chrono::Weekday::Wed),
            4 => Some(chrono::Weekday::Thu),
            5 => Some(chrono::Weekday::Fri),
            6 => Some(chrono::Weekday::Sat),
            7 => Some(chrono::Weekday::Sun),
            _ => None,
        };

        weekday.unwrap() == Local::now().weekday() && is_time
    });

    debug!(
        "Found {} schedule_entry instances that are current.",
        entries.len()
    );

    Ok(entries)
}

#[tracing::instrument]
pub fn current_classes(db: &Db) -> Result<Vec<Class>, QueryError> {
    db.transaction(|t| -> Result<Vec<Class>, QueryError> {
        let schedule_entries = get_current_schedule(db)?;
        if schedule_entries.is_empty() {
            return Ok(vec![]);
        };

        // TODO: find out why it won't work with key restriction

        let class: Vec<Class> = t
            .exec(
                QueryBuilder::select()
                    .elements::<Class>()
                    .search()
                    .to(QueryId::from(schedule_entries[0].db_id.unwrap()))
                    .where_()
                    .distance(agdb::CountComparison::Equal(2))
                    .query(),
            )?
            .try_into()?;

        Ok(class)
    })
}
