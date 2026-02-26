use serde_json::{json, Value,Error};// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub fn json_beauty(data_str: &str) -> String {
    // Format the input string
    let value: serde_json::Value= match serde_json::from_str(data_str) {
        Ok(value) => value,
        Err(e) => return format!("Err: {}",e).to_string(),
    };
    // Pretty-print JSON value
    let pretty: String = match serde_json::to_string_pretty(&value).map_err(|e| format!("Error formatting JSON: {}", e)){
        Ok(value) => return value,
        Err(e) => return format!("Err: {}",e).to_string(),
    };

    println!("{}", pretty);

    pretty
}

#[tauri::command]
fn beautify(data_str: &str, file_type: &str) -> String {
    match file_type {
        "json" => return json_beauty(data_str),
        // "xml" => return xml_beauty(data_str),
        // "yaml" | "yml" => return yaml_beauty(data_str),
        _ => data_str.to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![beautify])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
