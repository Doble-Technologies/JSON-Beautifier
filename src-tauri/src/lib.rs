use serde_json::{json, Value};// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


#[tauri::command]
fn beautify(data_str: &str) -> String {
    //Format the input string
    let value: Value = serde_json::from_str(data_str).unwrap();
    // Pretty-print JSON value
    let pretty = serde_json::to_string_pretty(&value).unwrap();

    pretty



}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![beautify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
