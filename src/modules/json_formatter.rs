use serde_json::Value;

pub fn format_json(input: &str) -> String {
    match serde_json::from_str::<Value>(input) {
        Ok(json) => {
            serde_json::to_string_pretty(&json)
                .unwrap_or_else(|_| String::from("Error formatting JSON"))
        }
        Err(_) => String::from("Invalid JSON")
    }
} 