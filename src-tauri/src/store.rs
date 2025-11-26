use serde_json::json;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";
const API_KEY_RECORD: &str = "resend_api_key";

#[tauri::command]
pub fn has_api_key(app: AppHandle<Wry>) -> Result<bool, String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    let has_key = store.get(API_KEY_RECORD).is_some();

    Ok(has_key)
}

#[tauri::command]
pub fn save_api_key(app: AppHandle<Wry>, api_key: String) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    store.set(API_KEY_RECORD, json!(api_key));

    store
        .save()
        .map_err(|e| format!("[ERROR] Failed to save store: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_api_key(app: AppHandle<Wry>) -> Result<Option<String>, String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    match store.get(API_KEY_RECORD) {
        Some(value) => {
            let api_key = value
                .as_str()
                .ok_or_else(|| "[ERROR] API key is not a string".to_string())?
                .to_string();

            println!("[INFO] API key retrieved successfully");
            Ok(Some(api_key))
        }
        None => {
            println!("[INFO]  API key not found");
            Ok(None)
        }
    }
}

#[tauri::command]
pub fn delete_api_key(app: AppHandle<Wry>) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("ERROR Failed to load store: {}", e))?;

    let deleted = store.delete(API_KEY_RECORD);

    if !deleted {
        println!("[WARN] API key was not found in store");
    }

    store
        .save()
        .map_err(|e| format!("ERROR Failed to save store: {}", e))?;

    println!("[INFO] API key deleted successfully");

    Ok(())
}
