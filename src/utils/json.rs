use serde_json::{from_str, to_string_pretty, Value};

/// 格式化JSON字符串
pub fn format_json(json_str: &str) -> String {
    match from_str::<Value>(json_str) {
        Ok(json) => to_string_pretty(&json).unwrap_or_else(|_| String::from("JSON格式化失败")),
        Err(_) => String::from("无效的JSON格式"),
    }
}

/// 压缩JSON字符串
pub fn minify_json(json_str: &str) -> String {
    match from_str::<Value>(json_str) {
        Ok(json) => serde_json::to_string(&json).unwrap_or_else(|_| String::from("JSON压缩失败")),
        Err(_) => String::from("无效的JSON格式"),
    }
}
