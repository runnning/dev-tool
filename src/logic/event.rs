use slint::{Weak, SharedString};
use crate::MainWindow;
use crate::logic::time::TimeLogic;
use crate::logic::json::JsonLogic;

pub struct EventHandler {
    window: Weak<MainWindow>,
    time_logic: TimeLogic,
    json_logic: JsonLogic,
}

impl EventHandler {
    pub fn new(window: Weak<MainWindow>) -> Self {
        let window_clone = window.clone();
        Self {
            window: window.clone(),
            time_logic: TimeLogic::new(window_clone),
            json_logic: JsonLogic::new(),
        }
    }

    pub fn handle_events(&self) {
        // 处理时间相关事件
        self.handle_time_events();
        // 处理JSON相关事件
        self.handle_json_events();
    }

    fn handle_time_events(&self) {
        if let Some(window) = self.window.upgrade() {
            let time_logic = self.time_logic.clone();
            let window_weak = self.window.clone();
            // 日期时间转时间戳
            window.on_convert_to_timestamp(move |datetime: SharedString| {
                if let Some(window) = window_weak.upgrade() {
                    let result = time_logic.convert_to_timestamp(&datetime);
                    window.set_result(result.into());
                }
            });

            let time_logic = self.time_logic.clone();
            let window_weak = self.window.clone();
            // 日期时间转毫秒时间戳
            window.on_convert_to_ms_timestamp(move |datetime: SharedString| {
                if let Some(window) = window_weak.upgrade() {
                    let result = time_logic.convert_to_ms_timestamp(&datetime);
                    window.set_result(result.into());
                }
            });

            let time_logic = self.time_logic.clone();
            let window_weak = self.window.clone();
            // 获取当前时间
            window.on_request_current_time(move || {
                if let Some(window) = window_weak.upgrade() {
                    let current_time = time_logic.get_current_time();
                    window.set_current_time(current_time.into());
                }
            });

            let time_logic = self.time_logic.clone();
            let window_weak = self.window.clone();
            // 时间格式变更
            window.on_time_format_changed(move |format: SharedString| {
                if let Some(window) = window_weak.upgrade() {
                    if let Err(e) = time_logic.set_time_format(format.to_string()) {
                        window.set_result(format!("格式错误: {}", e).into());
                    }
                }
            });
        }
    }

    fn handle_json_events(&self) {
        if let Some(window) = self.window.upgrade() {
            let json_logic = self.json_logic.clone();
            let window_weak = self.window.clone();
            // JSON格式化
            window.on_format_json(move |input: SharedString| {
                if let Some(window) = window_weak.upgrade() {
                    let result = json_logic.format(&input);
                    window.set_output(result.into());
                }
            });

            let json_logic = self.json_logic.clone();
            let window_weak = self.window.clone();
            // JSON压缩
            window.on_minify_json(move |input: SharedString| {
                if let Some(window) = window_weak.upgrade() {
                    let result = json_logic.minify(&input);
                    window.set_output(result.into());
                }
            });
        }
    }
} 