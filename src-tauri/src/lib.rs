use quick_xml::events::Event;
use quick_xml::{Reader, Writer};
use std::io::Cursor;
use beautiful_md::{Config, format_markdown};
use serde_yaml::{self, Value};
use sqlformat::{format, FormatOptions, QueryParams};


pub fn json_beauty(data_str: &str) -> String {
    // Format the input string
    let value: serde_json::Value= match serde_json::from_str(data_str) {
        Ok(value) => value,
        Err(_e) => return format!("{}",data_str).to_string(),
    };

    //TODO: Debate/get feedback if we should display error or just return same
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

pub fn yml_beauty(input: &str) -> Result<String, String> {
    let value: Value = serde_yaml::from_str(input)
        .map_err(|e| format!("Failed to parse YAML: {e}"))?;

    let output = serde_yaml::to_string(&value)
        .map_err(|e| format!("Failed to serialize YAML: {e}"))?;

    Ok(output)
}

pub fn md_beauty(data_str: &str) ->  Result<String, String>{
    let config = Config::default();
    let markdown = &data_str.replace("\\n", "\n");
    let (formatted, _diagnostics) = format_markdown(markdown, &config).unwrap();
    Ok(formatted)
}

pub fn sql_beauty(data_str: &str) -> Result< String, String>{
    let opts = FormatOptions {
        indent: sqlformat::Indent::Spaces(2),
        uppercase: true,
        lines_between_queries: 1,
    };
    Ok(format(&data_str, &QueryParams::None, opts))
}

#[tauri::command]
fn beautify(data_str: &str, file_type: &str) -> String {
    match file_type {
        "json" => return json_beauty(data_str),
        "xml" => {
            if let Ok(pretty) = xml_beauty(data_str) {
                pretty
            } else if let Err(e) = xml_beauty(data_str) {
                e.to_string()
            } else {
                unreachable!()
            }
        },
        "yml" => {
            if let Ok(pretty) = yml_beauty(data_str) {
                pretty
            } else if let Err(e) = yml_beauty(data_str) {
                e
            } else {
                unreachable!()
            }
        },
        "markdown" => {
            if let Ok(pretty) =md_beauty(data_str) {
                pretty
            } else if let Err(e) = md_beauty(data_str) {
                e
            } else {
                unreachable!()
            }
        },
        "sql" => {
            if let Ok(pretty) =sql_beauty(data_str) {
                pretty
            } else if let Err(e) = sql_beauty(data_str) {
                e
            } else {
                unreachable!()
            }
        },
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
