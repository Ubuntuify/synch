use agdb::{Db, QueryBuilder, QueryError, QueryId};

use crate::structs::{Class, ScheduleTimeSlotEntry};

pub fn list_classes(db: &Db, limit: Option<u64>) -> Result<Vec<Class>, QueryError> {
    Ok(db
        .exec(
            QueryBuilder::select()
                .elements::<Class>()
                .search()
                .from(QueryId::from("class"))
                .limit(limit.unwrap_or(10))
                .where_()
                .distance(agdb::CountComparison::Equal(2))
                .query(),
        )?
        .try_into()?)
}

pub fn list_time_slots(
    db: &Db,
    limit: Option<u64>,
) -> Result<Vec<ScheduleTimeSlotEntry>, QueryError> {
    Ok(db
        .exec(
            QueryBuilder::select()
                .elements::<ScheduleTimeSlotEntry>()
                .search()
                .from(QueryId::from("schedule_entry"))
                .limit(limit.unwrap_or(10))
                .where_()
                .distance(agdb::CountComparison::Equal(2))
                .query(),
        )?
        .try_into()?)
}

