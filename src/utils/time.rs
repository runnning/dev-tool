use chrono::{DateTime, Local, NaiveDateTime};

/// 验证时间格式是否有效
pub fn validate_time_format(format: &str) -> Result<(), String> {
    // 检查格式字符串不能为空
    if format.is_empty() {
        return Err("时间格式不能为空".to_string());
    }

    // 检查格式字符串中的特殊字符
    let valid_chars = ['%', 'Y', 'y', 'm', 'd', 'H', 'M', 'S', '-', '/', '年', '月', '日', ':', ' ', '.', 'f'];
    for c in format.chars() {
        if !valid_chars.contains(&c) {
            return Err(format!("无效的时间格式字符: {}", c));
        }
    }

    // 检查格式中是否至少包含一个时间格式说明符
    let format_specifiers = ["%Y", "%y", "%m", "%d", "%H", "%M", "%S"];
    let has_specifier = format_specifiers.iter().any(|&spec| format.contains(spec));
    if !has_specifier {
        return Err("时间格式需要至少包含一个时间格式说明符 (%Y, %y, %m, %d, %H, %M, %S)".to_string());
    }

    // 尝试使用当前时间验证格式
    let now = Local::now();
    match now.format(format).to_string() {
        s if s.is_empty() => Err("时间格式无效".to_string()),
        _ => Ok(())
    }
}

/// 获取当前时间，使用指定格式
pub fn get_current_time_with_format(format: &str) -> String {
    Local::now().format(format).to_string()
}

/// 将秒级时间戳转换为日期时间字符串，使用指定格式
pub fn timestamp_to_datetime_with_format(ts: i64, format: &str) -> Option<String> {
    DateTime::from_timestamp(ts, 0).map(|dt| {
        dt.with_timezone(&Local)
            .format(format)
            .to_string()
    })
}

/// 将毫秒级时间戳转换为日期时间字符串，使用指定格式
pub fn ms_timestamp_to_datetime_with_format(ts: i64, format: &str) -> Option<String> {
    let seconds = ts / 1000;
    let nanos = ((ts % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(seconds, nanos).map(|dt| {
        dt.with_timezone(&Local)
            .format(format)
            .to_string()
    })
}

/// 将日期时间字符串转换为秒级时间戳，使用指定格式
pub fn datetime_to_timestamp_with_format(datetime_str: &str, format: &str) -> Result<i64, String> {
    NaiveDateTime::parse_from_str(datetime_str, format)
        .map(|dt| {
            DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()).timestamp()
        })
        .map_err(|e| format!("无效的日期格式: {}", e))
}

/// 将日期时间字符串转换为毫秒级时间戳，使用指定格式
pub fn datetime_to_ms_timestamp_with_format(datetime_str: &str, format: &str) -> Result<i64, String> {
    NaiveDateTime::parse_from_str(datetime_str, format)
        .map(|dt| {
            let local_dt = DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset());
            local_dt.timestamp() * 1000 + local_dt.timestamp_subsec_millis() as i64
        })
        .map_err(|e| format!("无效的日期格式: {}", e))
}

/// 解析时间戳字符串为i64
pub fn parse_timestamp(timestamp_str: &str) -> Result<i64, &'static str> {
    timestamp_str.parse::<i64>().map_err(|_| "无效的时间戳格式")
}

// 为了向后兼容，保留原有的函数
pub fn get_current_time() -> String {
    get_current_time_with_format("%Y-%m-%d %H:%M:%S")
}

pub fn timestamp_to_datetime(ts: i64) -> Option<String> {
    timestamp_to_datetime_with_format(ts, "%Y-%m-%d %H:%M:%S")
}

pub fn ms_timestamp_to_datetime(ts: i64) -> Option<String> {
    ms_timestamp_to_datetime_with_format(ts, "%Y-%m-%d %H:%M:%S.%3f")
}

pub fn datetime_to_timestamp(datetime_str: &str) -> Result<i64, String> {
    datetime_to_timestamp_with_format(datetime_str, "%Y-%m-%d %H:%M:%S")
}

pub fn datetime_to_ms_timestamp(datetime_str: &str) -> Result<i64, String> {
    datetime_to_ms_timestamp_with_format(datetime_str, "%Y-%m-%d %H:%M:%S")
}
