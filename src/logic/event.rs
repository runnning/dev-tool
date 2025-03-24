use slint::{Weak, SharedString};
use crate::{MainWindow, logic::{time::TimeLogic, json::JsonLogic}};
use crate::services::{config::ConfigService, storage::StorageService};

pub struct EventHandler {
    window: Weak<MainWindow>,
    time_logic: TimeLogic,
    json_logic: JsonLogic,
    config_service: ConfigService,
    storage_service: StorageService,
}

impl EventHandler {
    pub fn new(window: Weak<MainWindow>) -> Self {
        Self {
            window,
            time_logic: TimeLogic,
            json_logic: JsonLogic,
            config_service: ConfigService::new(),
            storage_service: StorageService::new(),
        }
    }

    pub fn setup(&self) {
        self.setup_time_handlers();
        self.setup_json_handlers();
    }

    fn setup_time_handlers(&self) {
        self.setup_current_time_handler();
        self.setup_timestamp_conversion_handlers();
        self.setup_datetime_conversion_handlers();
    }

    fn setup_current_time_handler(&self) {
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_request_current_time(move || {
                let current_time = TimeLogic::get_current_time();
                if let Some(window) = window.upgrade() {
                    window.set_current_time(current_time.into());
                }
            });
        }).unwrap();
    }

    fn setup_timestamp_conversion_handlers(&self) {
        let window = self.window.clone();
        let time_logic = self.time_logic;

        // 秒级时间戳转换
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_timestamp(move |input: SharedString| {
                let result = time_logic.convert_timestamp(input.as_str());
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();

        let window = self.window.clone();
        let time_logic = self.time_logic;

        // 毫秒时间戳转换
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_ms_timestamp(move |input: SharedString| {
                let result = time_logic.convert_ms_timestamp(input.as_str());
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();
    }

    fn setup_datetime_conversion_handlers(&self) {
        let window = self.window.clone();
        let time_logic = self.time_logic;

        // 日期时间转秒级时间戳
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_to_timestamp(move |input: SharedString| {
                let result = time_logic.convert_to_timestamp(input.as_str());
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();

        let window = self.window.clone();
        let time_logic = self.time_logic;

        // 日期时间转毫秒时间戳
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_to_ms_timestamp(move |input: SharedString| {
                let result = time_logic.convert_to_ms_timestamp(input.as_str());
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();
    }

    fn setup_json_handlers(&self) {
        self.setup_json_format_handler();
        self.setup_json_minify_handler();
    }

    fn setup_json_format_handler(&self) {
        let window = self.window.clone();
        let json_logic = self.json_logic;

        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_format_json(move |input: SharedString| {
                let result = json_logic.format(input.as_str());
                if let Some(window) = window.upgrade() {
                    window.set_output(result.into());
                }
            });
        }).unwrap();
    }

    fn setup_json_minify_handler(&self) {
        let window = self.window.clone();
        let json_logic = self.json_logic;

        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_minify_json(move |input: SharedString| {
                let result = json_logic.minify(input.as_str());
                if let Some(window) = window.upgrade() {
                    window.set_output(result.into());
                }
            });
        }).unwrap();
    }

    pub fn init(&self) {
        // 加载配置，后续可以用于主题等设置
        let _config = self.config_service.load();
        
        // 设置时间工具事件处理
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_request_current_time(move || {
                let result = TimeLogic::get_current_time();
                storage.save_history("time", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_current_time(result.into());
                }
            });
        }).unwrap();

        let time_logic = self.time_logic;
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_timestamp(move |timestamp: SharedString| {
                let result = time_logic.convert_timestamp(&timestamp);
                storage.save_history("time", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();

        let time_logic = self.time_logic;
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_ms_timestamp(move |timestamp: SharedString| {
                let result = time_logic.convert_ms_timestamp(&timestamp);
                storage.save_history("time", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();

        let time_logic = self.time_logic;
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_to_timestamp(move |datetime: SharedString| {
                let result = time_logic.convert_to_timestamp(&datetime);
                storage.save_history("time", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();

        let time_logic = self.time_logic;
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_convert_to_ms_timestamp(move |datetime: SharedString| {
                let result = time_logic.convert_to_ms_timestamp(&datetime);
                storage.save_history("time", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_result(result.into());
                }
            });
        }).unwrap();

        // 设置JSON工具事件处理
        let json_logic = self.json_logic;
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_format_json(move |input: SharedString| {
                let result = json_logic.format(&input);
                storage.save_history("json", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_output(result.into());
                }
            });
        }).unwrap();

        let json_logic = self.json_logic;
        let storage = self.storage_service.clone();
        let window = self.window.clone();
        self.window.upgrade_in_event_loop(move |handle| {
            handle.on_minify_json(move |input: SharedString| {
                let result = json_logic.minify(&input);
                storage.save_history("json", &result).ok();
                if let Some(window) = window.upgrade() {
                    window.set_output(result.into());
                }
            });
        }).unwrap();
    }
} 