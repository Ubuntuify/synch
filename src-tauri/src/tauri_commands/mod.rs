use agdb::Db;
use std::sync::Mutex;
use tauri::State;

use crate::structs::Class;

#[tauri::command]
pub fn get_current_classes(state: State<'_, Mutex<Db>>) -> Vec<Class> {
    let state = state.lock().unwrap();
    crate::user::current_classes(&*state).unwrap()
}
