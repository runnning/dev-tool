use slint::*;

include_modules!();

use chrono::{Local, TimeZone};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), PlatformError> {
    let main_window = MainWindow::new()?;

    // 获取当前时间戳
    let main_window_weak = main_window.as_weak();
    main_window.on_get_current_timestamp(move || {
        let main_window = main_window_weak.unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        main_window.set_timestamp_input(now.as_secs().to_string().into());
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_get_current_ms_timestamp(move || {
        let main_window = main_window_weak.unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        main_window.set_timestamp_input(now.as_millis().to_string().into());
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_convert_timestamp(move || {
        let main_window = main_window_weak.unwrap();
        let input = main_window.get_timestamp_input();
        if let Ok(timestamp) = input.parse::<i64>() {
            let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
            main_window.set_timestamp_output(datetime.format("%Y-%m-%d %H:%M:%S").to_string().into());
        } else {
            main_window.set_timestamp_output(String::from("Invalid timestamp").into());
        }
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_convert_ms_timestamp(move || {
        let main_window = main_window_weak.unwrap();
        let input = main_window.get_timestamp_input();
        if let Ok(timestamp) = input.parse::<i64>() {
            let datetime = Local.timestamp_millis_opt(timestamp).unwrap();
            main_window.set_timestamp_output(datetime.format("%Y-%m-%d %H:%M:%S.%3f").to_string().into());
        } else {
            main_window.set_timestamp_output(String::from("Invalid timestamp").into());
        }
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_format_json(move || {
        let main_window = main_window_weak.unwrap();
        let input = main_window.get_json_input();
        match serde_json::from_str::<Value>(&input) {
            Ok(json) => {
                main_window.set_json_output(serde_json::to_string_pretty(&json).unwrap_or_else(|_| String::from("Error formatting JSON")).into());
            }
            Err(_) => {
                main_window.set_json_output(String::from("Invalid JSON").into());
            }
        }
    });

    main_window.run()
}