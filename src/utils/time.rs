use chrono::{DateTime, Local, NaiveDateTime};

/// 获取当前时间，格式：YYYY-MM-DD HH:mm:ss
pub fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 将秒级时间戳转换为日期时间字符串
pub fn timestamp_to_datetime(ts: i64) -> Option<String> {
    DateTime::from_timestamp(ts, 0).map(|dt| {
        dt.with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    })
}

/// 将毫秒级时间戳转换为日期时间字符串
pub fn ms_timestamp_to_datetime(ts: i64) -> Option<String> {
    let seconds = ts / 1000;
    let nanos = ((ts % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(seconds, nanos).map(|dt| {
        dt.with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S.%3f")
            .to_string()
    })
}

/// 将日期时间字符串转换为秒级时间戳
pub fn datetime_to_timestamp(datetime_str: &str) -> Result<i64, &'static str> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .map(|dt| {
            DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()).timestamp()
        })
        .map_err(|_| "无效的日期格式")
}

/// 将日期时间字符串转换为毫秒级时间戳
pub fn datetime_to_ms_timestamp(datetime_str: &str) -> Result<i64, &'static str> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .map(|dt| {
            let local_dt = DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset());
            local_dt.timestamp() * 1000 + local_dt.timestamp_subsec_millis() as i64
        })
        .map_err(|_| "无效的日期格式")
}

/// 解析时间戳字符串为i64
pub fn parse_timestamp(timestamp_str: &str) -> Result<i64, &'static str> {
    timestamp_str.parse::<i64>().map_err(|_| "无效的时间戳格式")
}
