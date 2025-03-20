use chrono::{Local, TimeZone};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    now.as_secs().to_string()
}

pub fn get_current_ms_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    now.as_millis().to_string()
}

pub fn convert_timestamp(timestamp: &str) -> String {
    match timestamp.parse::<i64>() {
        Ok(ts) => {
            match Local.timestamp_opt(ts, 0) {
                chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                _ => String::from("Invalid timestamp")
            }
        }
        Err(_) => String::from("Invalid timestamp")
    }
}

pub fn convert_ms_timestamp(timestamp: &str) -> String {
    match timestamp.parse::<i64>() {
        Ok(ts) => {
            match Local.timestamp_millis_opt(ts) {
                chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S.%3f").to_string(),
                _ => String::from("Invalid timestamp")
            }
        }
        Err(_) => String::from("Invalid timestamp")
    }
} 