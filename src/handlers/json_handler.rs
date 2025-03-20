use slint::ComponentHandle;
use crate::{MainWindow, modules::*};

pub struct JsonHandler;

impl JsonHandler {
    pub fn setup(main_window: &MainWindow) {
        let main_window_weak = main_window.as_weak();
        main_window.on_format_json(move || {
            let main_window = main_window_weak.clone();
            if let Some(window) = main_window.upgrade() {
                let input = window.get_json_input().to_string();
                
                // 检查输入是否为空
                if input.trim().is_empty() {
                    window.set_json_output("请输入要格式化的 JSON 文本".into());
                    return;
                }
                
                // 显示处理中的状态
                window.set_json_output("正在处理...".into());
                
                // 异步处理 JSON
                format_json_async(main_window, input);
            }
        });
    }
} 