use serde_json::Value;
use std::thread;
use slint::Weak;
use crate::MainWindow;

pub fn format_json_async(window: Weak<MainWindow>, input: String) {
    thread::spawn(move || {
        // 添加延迟以确保 UI 能显示处理中的状态
        thread::sleep(std::time::Duration::from_millis(50));

        let result = match serde_json::from_str::<Value>(&input) {
            Ok(json) => {
                serde_json::to_string_pretty(&json)
                    .unwrap_or_else(|_| String::from("Error formatting JSON"))
            }
            Err(_) => String::from("无效的 JSON 格式")
        };

        // 在主线程中更新 UI
        slint::invoke_from_event_loop(move || {
            if let Some(window) = window.upgrade() {
                window.set_json_output(result.into());
            }
        }).ok();
    });
} 