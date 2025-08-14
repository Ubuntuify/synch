use agdb::{Db, QueryBuilder, QueryError, QueryId};
use chrono::{Local, NaiveTime};
use tracing::{debug, info};

use crate::structs::{Class, ScheduleTimeSlotEntry};

#[tracing::instrument]
fn get_current_schedule(db: &Db) -> Result<Vec<ScheduleTimeSlotEntry>, QueryError> {
    info!("Starting search for current schedule time slot.");

    let mut schedule_entries = crate::db::list_time_slots(db, None)?;

    debug!(
        "Found {} schedule entries available, searching through them.",
        schedule_entries.len()
    );

    schedule_entries.retain(|t| {
        let current_time = Local::now().time();
        let start_time = NaiveTime::parse_from_str(&t.start_time, t.time_format()).unwrap();
        let end_time = NaiveTime::parse_from_str(&t.end_time, t.time_format()).unwrap();

        start_time <= current_time && current_time < end_time
    });

    debug!(
        "Found {} schedule_entry instances that are current.",
        schedule_entries.len()
    );

    Ok(schedule_entries)
}

#[tracing::instrument]
pub fn current_classes(db: &Db) -> Result<Vec<Class>, QueryError> {
    db.transaction(|t| -> Result<Vec<Class>, QueryError> {
        let schedule_entries = get_current_schedule(db)?;
        if schedule_entries.is_empty() {
            return Ok(vec![]);
        };

        // TODO: find out why it won't work with key restriction

        let class = t
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

        Ok(vec![class])
    })
}
