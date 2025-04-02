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
                    match time_logic.set_time_format(format.to_string()) {
                        Ok(()) => {
                            // 格式设置成功
                            println!("时间格式已更新为: {}", format);
                        },
                        Err(e) => {
                            window.set_result(format!("格式错误: {}", e).into());
                        }
                    }
                }
            });
            
            // 监听格式索引变化
            let time_logic_clone = self.time_logic.clone();
            let weak_window = self.window.clone();
            
            window.on_current_format_index_changed(move |idx| {
                if let Some(window_instance) = weak_window.upgrade() {
                    // 获取对应索引的格式字符串
                    let format = match idx {
                        0 => "%Y-%m-%d %H:%M:%S",
                        1 => "%Y/%m/%d %H:%M:%S",
                        2 => "%Y年%m月%d日 %H:%M:%S",
                        3 => "%Y@%m@%d %H:%M:%S",
                        4 => "%Y-%m-%d",
                        5 => "%Y/%m/%d",
                        6 => "%Y年%m月%d日",
                        7 => "%Y@%m@%d",
                        8 => "%H:%M:%S",
                        9 => "%H:%M",
                        10 => "%Y-%m-%d %H:%M",
                        11 => "%Y/%m/%d %H:%M",
                        12 => "%Y@%m@%d %H:%M",
                        13 => "%m-%d %H:%M",
                        14 => "%m/%d %H:%M",
                        15 => "%m@%d %H:%M",
                        16 => "%Y-%m-%d %H:%M:%S.%3f",
                        17 => "%Y@%m@%d %H:%M:%S.%3f",
                        _ => "%Y-%m-%d %H:%M:%S", // 默认格式
                    };
                    
                    match time_logic_clone.set_time_format(format.to_string()) {
                        Ok(()) => {
                            window_instance.set_time_format(format.into());
                            println!("索引 {} 对应的格式已应用: {}", idx, format);
                        },
                        Err(e) => {
                            window_instance.set_result(format!("格式错误: {}", e).into());
                        }
                    }
                }
            });

            // 初始化时应用默认格式
            if let Some(_window) = self.window.upgrade() {
                let format = "%Y-%m-%d %H:%M:%S".to_string();
                match self.time_logic.set_time_format(format.clone()) {
                    Ok(()) => {
                        println!("初始时间格式设置为: {}", format);
                    },
                    Err(e) => {
                        eprintln!("初始化格式错误: {}", e);
                    }
                }
            }
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