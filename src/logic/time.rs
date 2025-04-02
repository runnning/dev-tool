use slint::{Weak, SharedString};
use crate::utils::time;
use crate::MainWindow;
use std::cell::RefCell;

#[derive(Clone)]
pub struct TimeLogic {
    window: Weak<MainWindow>,
    time_format: RefCell<String>,
}

impl TimeLogic {
    pub fn new(window: Weak<MainWindow>) -> Self {
        Self {
            window,
            time_format: RefCell::new("%Y-%m-%d %H:%M:%S".to_string()),
        }
    }

    pub fn request_current_time(&self) {
        if let Some(window) = self.window.upgrade() {
            let current_time = time::get_current_time();
            window.set_current_time(SharedString::from(current_time));
        }
    }

    pub fn convert_timestamp(&self, timestamp: &str) -> String {
        if let Ok(ts) = timestamp.parse::<i64>() {
            let format = self.time_format.borrow().clone();
            if let Some(result) = time::timestamp_to_datetime_with_format(ts, &format) {
                return result;
            }
        }
        "转换失败".to_string()
    }

    pub fn convert_ms_timestamp(&self, timestamp: &str) -> String {
        if let Ok(ts) = timestamp.parse::<i64>() {
            let format = self.time_format.borrow().clone();
            if let Some(result) = time::ms_timestamp_to_datetime_with_format(ts, &format) {
                return result;
            }
        }
        "转换失败".to_string()
    }

    pub fn convert_to_timestamp(&self, datetime: &str) -> String {
        if datetime.trim().is_empty() {
            return "请输入日期时间".to_string();
        }
        
        let format = self.time_format.borrow().clone();
        match time::datetime_to_timestamp_with_format(datetime, &format) {
            Ok(result) => result.to_string(),
            Err(e) => format!("转换失败: {}。请按格式 {} 输入", e, format)
        }
    }

    pub fn convert_to_ms_timestamp(&self, datetime: &str) -> String {
        if datetime.trim().is_empty() {
            return "请输入日期时间".to_string();
        }
        
        let format = self.time_format.borrow().clone();
        match time::datetime_to_ms_timestamp_with_format(datetime, &format) {
            Ok(result) => result.to_string(),
            Err(e) => format!("转换失败: {}。请按格式 {} 输入", e, format)
        }
    }

    pub fn set_time_format(&self, format: String) -> Result<(), String> {
        time::validate_time_format(&format)?;
        *self.time_format.borrow_mut() = format;
        Ok(())
    }

    pub fn get_time_format(&self) -> String {
        self.time_format.borrow().clone()
    }

    pub fn validate_time_format(format: &str) -> Result<(), String> {
        time::validate_time_format(format)
    }

    pub fn update_format_preview(&self, format: &str) -> Result<String, String> {
        time::validate_time_format(format)?;
        Ok(time::get_current_time_with_format(format))
    }

    pub fn get_current_time(&self) -> String {
        time::get_current_time()
    }
}
