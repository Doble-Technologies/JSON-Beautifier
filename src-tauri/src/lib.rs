use quick_xml::events::Event;
use quick_xml::{Reader, Writer};
use std::io::Cursor;

pub fn json_beauty(data_str: &str) -> String {
    // Format the input string
    let value: serde_json::Value= match serde_json::from_str(data_str) {
        Ok(value) => value,
        Err(_e) => return format!("{}",data_str).to_string(),
    };

    //TODO: Update to match xml error handling
    return match serde_json::to_string_pretty(&value).map_err(|e| format!("Error formatting JSON: {}", e)) {
        Ok(value) => value,
        Err(_e) => format!("{}", data_str).to_string(),
    };
}

pub fn xml_beauty(data_str: &str) -> Result<String, quick_xml::Error> {
    // Format the input string
    let mut reader = Reader::from_str(data_str);
    reader.config_mut().trim_text(true); // strip existing whitespace/indentation

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(event) => writer.write_event(event)?,
            Err(e) => return Err(e),
        }
    }

    let bytes = writer.into_inner().into_inner();
    Ok(String::from_utf8(bytes).expect("Valid UTF-8"))
}

#[tauri::command]
fn beautify(data_str: &str, file_type: &str) -> String {
    match file_type {
        "json" => return json_beauty(data_str),
        "xml" => {
            match xml_beauty(data_str) {
                Ok(pretty) => pretty,
                Err(e) => e.to_string(),
            }
        },
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
