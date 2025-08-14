use agdb::{Db, DbId, QueryBuilder, QueryError, QueryId};
use tracing::info;

use crate::{
    db::assign::RelationshipType,
    structs::{Class, ScheduleTimeSlotEntry},
};

#[tracing::instrument]
pub fn create_class(db: &mut Db, class: Class) -> Result<DbId, QueryError> {
    db.transaction_mut(|t| -> Result<DbId, QueryError> {
        info!("Inserting '{}' class entry to database.", class.name);

        let class = t
            .exec_mut(QueryBuilder::insert().element(&class).query())?
            .elements[0]
            .id;

        t.exec_mut(
            QueryBuilder::insert()
                .edges()
                .from(QueryId::from("class"))
                .to(class)
                .values([vec![]])
                .query(),
        )?;

        Ok(class)
    })
}

#[tracing::instrument]
pub fn create_schedule_entry(
    db: &mut Db,
    class: DbId,
    entry: ScheduleTimeSlotEntry,
) -> Result<DbId, QueryError> {
    db.transaction_mut(|t| -> Result<DbId, QueryError> {
        let entry = t
            .exec_mut(QueryBuilder::insert().element(&entry).query())?
            .elements[0]
            .id;

        t.exec_mut(
            QueryBuilder::insert()
                .edges()
                .from([QueryId::from("schedule_entry"), class.into()])
                .to(entry)
                .values([
                    vec![],
                    vec![(RelationshipType::ScheduleClass.convert_db(), 1_u64).into()],
                ])
                .query(),
        )?;

        Ok(entry)
    })
}
