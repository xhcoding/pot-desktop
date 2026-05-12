use crate::config::{get, set};
use crate::window::updater_window;

pub fn check_update(_app_handle: tauri::AppHandle) {
    let enable = match get("check_update") {
        Some(v) => v.as_bool().unwrap(),
        None => {
            set("check_update", true);
            true
        }
    };
    if enable {
        updater_window();
    }
}
