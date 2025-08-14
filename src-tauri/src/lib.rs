use agdb::{Db, QueryBuilder, QueryError};
use tracing::{event, Level};

pub mod db;
pub mod structs;
pub mod user;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![/* your commands here */])
        // You can add plugins here, for example:
        // .plugin(tauri_plugin_deep_link::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let mut db = Db::new("schedule.agdb").unwrap();
    crate::init_db(&mut db).expect("database was not able to be initialized");
}

#[tracing::instrument]
pub fn init_db(db: &mut Db) -> Result<(), QueryError> {
    event!(
        Level::INFO,
        "Beginning database init. Creating root node and creating aliases."
    );

    db.transaction_mut(|t| -> Result<(), QueryError> {
        t.exec_mut(
            QueryBuilder::insert()
                .nodes()
                .aliases(["root", "schedule_entry", "class", "room"])
                .query(),
        )?;

        t.exec_mut(
            QueryBuilder::insert()
                .edges()
                .from("root")
                .to(["schedule_entry", "class", "room"])
                .query(),
        )?;

        Ok(())
    })?;

    Ok(())
}
