use agdb::{Db, DbId, QueryBuilder, QueryError};

pub enum RelationshipType {
    ScheduleClass,
    RoomOccupied,
}

impl RelationshipType {
    pub fn convert_db(&self) -> &str {
        match self {
            Self::ScheduleClass => "schedule_class",
            Self::RoomOccupied => "room_occupied",
        }
    }
}

pub fn assign_room(db: &mut Db, obj: DbId, room: DbId) -> Result<(), QueryError> {
    db.transaction_mut(|t| -> Result<(), QueryError> {
        t.exec_mut(
            QueryBuilder::insert()
                .edges()
                .from([obj])
                .to(room)
                .values([vec![
                    (RelationshipType::RoomOccupied.convert_db(), 1_u64).into(),
                ]])
                .query(),
        )?;

        Ok(())
    })
}

pub fn assign_class(db: &mut Db, class: DbId, schedule_entry: DbId) -> Result<(), QueryError> {
    db.transaction_mut(|t| -> Result<(), QueryError> {
        t.exec_mut(
            QueryBuilder::insert()
                .edges()
                .from([class])
                .to(schedule_entry)
                .values([vec![
                    (RelationshipType::ScheduleClass.convert_db(), 1_u64).into(),
                ]])
                .query(),
        )?;

        Ok(())
    })
}
