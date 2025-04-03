use crate::utils::time;
use std::cell::RefCell;

#[derive(Clone)]
pub struct TimeLogic {
    time_format: RefCell<String>,
}

impl TimeLogic {
    pub fn new() -> Self {
        Self {
            time_format: RefCell::new("%Y-%m-%d %H:%M:%S".to_string()),
        }
    }

    pub fn convert_to_timestamp(&self, datetime: &str) -> String {
        if datetime.trim().is_empty() {
            return "请输入日期时间".to_string();
        }

        let format = self.time_format.borrow().clone();
        match time::datetime_to_timestamp_with_format(datetime, &format) {
            Ok(result) => format!("秒级时间戳: {}", result),
            Err(e) => format!("转换失败: {}。请按格式 \"{}\" 输入", e, format),
        }
    }

    pub fn convert_to_ms_timestamp(&self, datetime: &str) -> String {
        if datetime.trim().is_empty() {
            return "请输入日期时间".to_string();
        }

        let format = self.time_format.borrow().clone();
        match time::datetime_to_ms_timestamp_with_format(datetime, &format) {
            Ok(result) => format!("毫秒级时间戳: {}", result),
            Err(e) => format!("转换失败: {}。请按格式 \"{}\" 输入", e, format),
        }
    }

    pub fn set_time_format(&self, format: String) -> Result<(), String> {
        time::validate_time_format(&format)?;
        *self.time_format.borrow_mut() = format;
        Ok(())
    }

    pub fn get_current_time(&self) -> String {
        time::get_current_time()
    }
}
