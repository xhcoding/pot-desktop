use crate::config::{get, set};
use crate::window::{input_translate, ocr_recognize, ocr_translate, selection_translate};
use crate::APP;
use log::{info, warn};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

fn normalize_shortcut(hotkey: &str) -> String {
    let parts: Vec<String> = hotkey.split('+').map(|s| s.trim().to_string()).collect();
    let mut normalized_parts = Vec::new();

    for part in &parts {
        let lower = part.to_lowercase();
        match lower.as_str() {
            "ctrl" | "control" => normalized_parts.push("control".to_string()),
            "alt" => normalized_parts.push("alt".to_string()),
            "shift" => normalized_parts.push("shift".to_string()),
            "cmd" | "meta" | "super" => normalized_parts.push("super".to_string()),
            key => {
                if key.len() == 1 {
                    normalized_parts.push(format!("key{}", key.to_lowercase()));
                } else {
                    normalized_parts.push(key.to_string());
                }
            }
        }
    }

    normalized_parts.join("+")
}

pub struct ShortcutRegistry {
    pub shortcuts: Mutex<HashMap<String, String>>, // hotkey_string -> action_name
}

impl ShortcutRegistry {
    pub fn new() -> Self {
        Self {
            shortcuts: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, hotkey: String, action: String) {
        let normalized = normalize_shortcut(&hotkey);
        info!(
            "insert shortcut: {} -> {} (normalized: {})",
            hotkey, action, normalized
        );
        let mut shortcuts = self.shortcuts.lock().unwrap();
        shortcuts.insert(normalized, action);
    }

    pub fn remove(&self, hotkey: &str) {
        let normalized = normalize_shortcut(hotkey);
        let mut shortcuts = self.shortcuts.lock().unwrap();
        shortcuts.remove(&normalized);
    }

    pub fn get_action(&self, hotkey: &str) -> Option<String> {
        let normalized = normalize_shortcut(hotkey);
        info!("get_action for: {} (normalized: {})", hotkey, normalized);
        let shortcuts = self.shortcuts.lock().unwrap();
        shortcuts.get(&normalized).cloned()
    }
}

fn register(app_handle: &AppHandle, name: &str, key: &str) -> Result<(), String> {
    let hotkey_str = {
        if key.is_empty() {
            match get(name) {
                Some(v) => v.as_str().unwrap().to_string(),
                None => {
                    set(name, "");
                    String::new()
                }
            }
        } else {
            key.to_string()
        }
    };

    if !hotkey_str.is_empty() {
        let shortcut = match Shortcut::from_str(&hotkey_str) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to parse shortcut '{}': {:?}", hotkey_str, e);
                return Err(format!("Invalid shortcut format: {}", hotkey_str));
            }
        };

        let normalized_hotkey = normalize_shortcut(&hotkey_str);
        if let Some(old_hotkey) = {
            let registry = app_handle.state::<ShortcutRegistry>();
            let shortcuts = registry.shortcuts.lock().unwrap();
            shortcuts
                .iter()
                .find_map(|(k, v)| if v == name { Some(k.clone()) } else { None })
        } {
            if old_hotkey != normalized_hotkey {
                let _ = app_handle.global_shortcut().unregister(old_hotkey.as_str());
                let registry = app_handle.state::<ShortcutRegistry>();
                registry.remove(&old_hotkey);
            }
        }

        match app_handle.global_shortcut().register(shortcut) {
            Ok(()) => {
                info!("Registered global shortcut: {} for {}", hotkey_str, name);
                let registry = app_handle.state::<ShortcutRegistry>();
                registry.insert(hotkey_str, name.to_string());
            }
            Err(e) => {
                warn!("Failed to register global shortcut: {} {:?}", hotkey_str, e);
                return Err(e.to_string());
            }
        };
    }
    Ok(())
}

pub fn register_shortcut(shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match shortcut {
        "hotkey_selection_translate" => register(app_handle, "hotkey_selection_translate", "")?,
        "hotkey_input_translate" => register(app_handle, "hotkey_input_translate", "")?,
        "hotkey_ocr_recognize" => register(app_handle, "hotkey_ocr_recognize", "")?,
        "hotkey_ocr_translate" => register(app_handle, "hotkey_ocr_translate", "")?,
        "all" => {
            register(app_handle, "hotkey_selection_translate", "")?;
            register(app_handle, "hotkey_input_translate", "")?;
            register(app_handle, "hotkey_ocr_recognize", "")?;
            register(app_handle, "hotkey_ocr_translate", "")?;
        }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
pub fn register_shortcut_by_frontend(name: &str, shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match name {
        "hotkey_selection_translate" => {
            register(app_handle, "hotkey_selection_translate", shortcut)?
        }
        "hotkey_input_translate" => register(app_handle, "hotkey_input_translate", shortcut)?,
        "hotkey_ocr_recognize" => register(app_handle, "hotkey_ocr_recognize", shortcut)?,
        "hotkey_ocr_translate" => register(app_handle, "hotkey_ocr_translate", shortcut)?,
        _ => {}
    }
    Ok(())
}

pub fn handle_shortcut(app_handle: &AppHandle, shortcut: &Shortcut) {
    let registry = app_handle.state::<ShortcutRegistry>();
    let hotkey_str = shortcut.into_string();
    if let Some(action) = registry.get_action(&hotkey_str) {
        info!("Shortcut triggered: {} -> {}", hotkey_str, action);
        match action.as_str() {
            "hotkey_selection_translate" => {
                info!("selection_translate");
                tauri::async_runtime::spawn(async move {
                    selection_translate();
                });
            }
            "hotkey_input_translate" => {
                tauri::async_runtime::spawn(async move {
                    input_translate();
                });
            }
            "hotkey_ocr_recognize" => {
                tauri::async_runtime::spawn(async move {
                    ocr_recognize();
                });
            }
            "hotkey_ocr_translate" => {
                tauri::async_runtime::spawn(async move {
                    ocr_translate();
                });
            }
            _ => {}
        }
    }
}
