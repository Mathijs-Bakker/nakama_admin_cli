use inquire::Editor;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

pub fn load_payload_templates(path: &str) -> HashMap<String, Value> {
    let data = fs::read_to_string(path).expect("Failed to read default_payloads.json");
    serde_json::from_str(&data).expect("Invalid JSON in default_payloads.json")
}

pub fn prompt_valid_json_for_rpc(rpc: &str, templates: &HashMap<String, Value>) -> String {
    let default_payload = templates.get(rpc).unwrap_or(&json!({})).to_string();

    loop {
        let edit_result = Editor::new("Edit JSON payload:")
            .with_predefined_text(&pretty_print(&default_payload))
            .prompt();

        match edit_result {
            Ok(contents) => {
                let trimmed = contents.trim();
                match serde_json::from_str::<Value>(trimmed) {
                    Ok(_) => return trimmed.to_string(),
                    Err(err) => {
                        eprintln!("❌ Invalid JSON: {err}. Re-opening editor...");
                    }
                }
            }
            Err(err) => {
                eprintln!("Editor error: {err}");
                std::process::exit(1);
            }
        }
    }
}

fn pretty_print(json_str: &str) -> String {
    serde_json::from_str::<Value>(json_str)
        .map(|v| serde_json::to_string_pretty(&v).unwrap_or_else(|_| json_str.to_string()))
        .unwrap_or_else(|_| json_str.to_string())
}
