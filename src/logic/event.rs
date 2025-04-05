use slint::{Weak, SharedString, Timer};
use crate::MainWindow;
use crate::logic::time::TimeLogic;
use crate::logic::json::JsonLogic;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;

pub struct EventHandler {
    window: Weak<MainWindow>,
    time_logic: TimeLogic,
    json_logic: JsonLogic,
}

impl EventHandler {
    pub fn new(window: Weak<MainWindow>) -> Self {
        Self {
            window: window.clone(),
            time_logic: TimeLogic::new(),
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
                    Self::apply_time_format(&time_logic, &window, &format);
                }
            });
            
            // 监听格式索引变化
            let time_logic_clone = self.time_logic.clone();
            let weak_window = self.window.clone();
            
            window.on_current_format_index_changed(move |idx| {
                if let Some(window_instance) = weak_window.upgrade() {
                    // 获取对应索引的格式字符串
                    let format = Self::get_time_format_by_index(idx);
                    
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

    fn get_time_format_by_index(idx: i32) -> &'static str {
        match idx {
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
        }
    }

    fn apply_time_format(time_logic: &TimeLogic, window: &MainWindow, format: &SharedString) {
        match time_logic.set_time_format(format.to_string()) {
            Ok(()) => {
                println!("时间格式已更新为: {}", format);
            },
            Err(e) => {
                window.set_result(format!("格式错误: {}", e).into());
            }
        }
    }

    fn handle_json_events(&self) {
        if let Some(window) = self.window.upgrade() {
            // JSON格式化
            let json_logic = self.json_logic.clone();
            let window_weak = self.window.clone();
            window.on_format_json(move |input: SharedString| {
                Self::process_json(&window_weak, &json_logic, &input, |logic, text| logic.format(text), "JSON格式化");
            });

            // JSON压缩
            let json_logic = self.json_logic.clone();
            let window_weak = self.window.clone();
            window.on_minify_json(move |input: SharedString| {
                Self::process_json(&window_weak, &json_logic, &input, |logic, text| logic.minify(text), "JSON压缩");
            });
        }
    }
    
    fn process_json<F>(
        window_weak: &Weak<MainWindow>, 
        json_logic: &JsonLogic, 
        input: &SharedString, 
        processor: F, 
        operation_name: &str
    ) where 
        F: Fn(&JsonLogic, &str) -> String + Send + 'static + Clone,
    {
        if let Some(window) = window_weak.upgrade() {
            // 设置处理状态
            window.set_json_processing(true);
            
            // 处理空输入
            if input.is_empty() {
                window.set_output("请输入JSON内容".into());
                window.set_json_processing(false);
                return;
            }
            
            // 对于小型JSON，直接同步处理
            if input.len() < 5000 {
                let result = processor(json_logic, input);
                // 设置输出结果
                window.set_output(result.into());
                // 确保处理状态被重置
                window.set_json_processing(false);
                return;
            }
            
            // 提示用户处理中
            window.set_output("正在处理中，请稍候...".into());
            
            // 创建一个通道用于接收处理结果
            let (sender, receiver) = mpsc::channel();
            let input_str = input.to_string();
            let json_logic_clone = json_logic.clone();
            let processor_clone = processor.clone();
            
            // 在新线程中处理大型JSON
            thread::spawn(move || {
                let result = processor_clone(&json_logic_clone, &input_str);
                let _ = sender.send(result);
            });
            
            // 创建一个定时器来检查处理结果
            let window_check = window_weak.clone();
            let start_time = Instant::now();
            let processing_tick = Rc::new(RefCell::new(0));
            let processing_tick_clone = processing_tick.clone();
            let operation = operation_name.to_string();
            
            // 每200毫秒更新一次处理状态显示
            Timer::default().start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(200),
                move || {
                    if let Some(window) = window_check.upgrade() {
                        // 尝试接收处理结果
                        if let Ok(result) = receiver.try_recv() {
                            // 计算处理耗时
                            let elapsed = start_time.elapsed();
                            println!("{}总耗时: {:?}", operation, elapsed);
                            
                            // 更新UI
                            window.set_output(result.into());
                            window.set_json_processing(false);
                            return;
                        }
                        
                        // 更新进度消息，让用户知道处理仍在进行
                        if window.get_json_processing() {
                            let mut counter = *processing_tick_clone.borrow();
                            counter = (counter + 1) % 4;
                            *processing_tick_clone.borrow_mut() = counter;
                            
                            let elapsed_secs = start_time.elapsed().as_secs();
                            let dots = ".".repeat(counter + 1);
                            let progress_msg = format!("正在处理中{}已用时{}秒", dots, elapsed_secs);
                            window.set_output(progress_msg.into());
                        }
                    }
                }
            );
            
            // 设置超时定时器
            let window_timeout = window_weak.clone();
            Timer::default().start(
                slint::TimerMode::SingleShot,
                std::time::Duration::from_secs(30), // 30秒超时
                move || {
                    if let Some(window) = window_timeout.upgrade() {
                        if window.get_json_processing() {
                            window.set_json_processing(false);
                            window.set_output("处理超时，JSON数据量可能过大或格式有误".into());
                        }
                    }
                }
            );
        }
    }
} 