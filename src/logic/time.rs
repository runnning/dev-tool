use crate::utils::time;

#[derive(Clone, Copy)]
pub struct TimeLogic;

impl TimeLogic {
    pub fn get_current_time() -> String {
        time::get_current_time()
    }

    pub fn convert_timestamp(&self, timestamp: &str) -> String {
        match time::parse_timestamp(timestamp) {
            Ok(ts) => time::timestamp_to_datetime(ts)
                .unwrap_or_else(|| "无效的时间戳".to_string()),
            Err(e) => e.to_string(),
        }
    }

    pub fn convert_ms_timestamp(&self, timestamp: &str) -> String {
        match time::parse_timestamp(timestamp) {
            Ok(ts) => time::ms_timestamp_to_datetime(ts)
                .unwrap_or_else(|| "无效的时间戳".to_string()),
            Err(e) => e.to_string(),
        }
    }

    pub fn convert_to_timestamp(&self, datetime: &str) -> String {
        match time::datetime_to_timestamp(datetime) {
            Ok(ts) => ts.to_string(),
            Err(e) => e.to_string(),
        }
    }

    pub fn convert_to_ms_timestamp(&self, datetime: &str) -> String {
        match time::datetime_to_ms_timestamp(datetime) {
            Ok(ts) => ts.to_string(),
            Err(e) => e.to_string(),
        }
    }
}
