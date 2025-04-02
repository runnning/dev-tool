use chrono::{DateTime, Local, NaiveDateTime};

/// 获取所有支持的日期时间格式
pub fn get_all_supported_formats() -> Vec<&'static str> {
    vec![
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%Y年%m月%d日 %H:%M:%S",
        "%Y@%m@%d %H:%M:%S",
        "%Y-%m-%d",
        "%Y/%m/%d",
        "%Y年%m月%d日",
        "%Y@%m@%d",
        "%H:%M:%S",
        "%H:%M",
        "%Y-%m-%d %H:%M",
        "%Y/%m/%d %H:%M",
        "%Y@%m@%d %H:%M",
        "%m-%d %H:%M",
        "%m/%d %H:%M",
        "%m@%d %H:%M",
        "%Y-%m-%d %H:%M:%S.%3f",
        "%Y@%m@%d %H:%M:%S.%3f"
    ]
}

/// 验证时间格式是否有效
pub fn validate_time_format(format: &str) -> Result<(), String> {
    // 检查格式字符串不能为空
    if format.is_empty() {
        return Err("时间格式不能为空".to_string());
    }

    // 检查格式字符串中的特殊字符
    let valid_chars = ['%', 'Y', 'y', 'm', 'd', 'H', 'M', 'S', '-', '/', '@', '年', '月', '日', ':', ' ', '.', 'f'];
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
    println!("尝试将 '{}' 转换为时间戳，使用格式 '{}'", datetime_str, format);
    
    // 智能判断输入的是日期、时间还是日期时间
    let has_date_chars = datetime_str.contains('-') || datetime_str.contains('/') || 
                        datetime_str.contains('年') || datetime_str.contains('@');
    let has_time_chars = datetime_str.contains(':');
    
    // 检查输入看起来像什么类型
    let looks_like_date_only = has_date_chars && !has_time_chars;
    let looks_like_time_only = !has_date_chars && has_time_chars;
    let looks_like_full_datetime = has_date_chars && has_time_chars;
    
    println!("输入分析: 看起来像日期={}, 看起来像时间={}, 看起来像完整日期时间={}", 
             looks_like_date_only, looks_like_time_only, looks_like_full_datetime);
    
    // 1. 如果输入看起来像纯日期，尝试使用日期格式解析
    if looks_like_date_only {
        println!("输入看起来像纯日期，尝试用日期格式解析");
        
        // 根据分隔符选择合适的格式
        let date_format = if datetime_str.contains('-') {
            "%Y-%m-%d"
        } else if datetime_str.contains('/') {
            "%Y/%m/%d"
        } else if datetime_str.contains('@') {
            "%Y@%m@%d"
        } else if datetime_str.contains('年') {
            "%Y年%m月%d日"
        } else {
            "%Y-%m-%d" // 默认格式
        };
        
        println!("使用日期格式: {}", date_format);
        
        match chrono::NaiveDate::parse_from_str(datetime_str, date_format) {
            Ok(date) => {
                println!("成功解析为日期: {:?}", date);
                let datetime = date.and_hms_opt(0, 0, 0).unwrap();
                let timestamp = DateTime::<Local>::from_naive_utc_and_offset(datetime, *Local::now().offset()).timestamp();
                println!("转换为时间戳: {}", timestamp);
                return Ok(timestamp);
            },
            Err(e) => {
                println!("纯日期解析失败: {:?}", e);
            }
        }
    }
    
    // 2. 如果输入看起来像纯时间，尝试使用时间格式解析
    if looks_like_time_only {
        println!("输入看起来像纯时间，尝试用时间格式解析");
        
        // 根据输入长度选择时间格式
        let time_format = if datetime_str.matches(':').count() == 1 {
            "%H:%M"
        } else {
            "%H:%M:%S"
        };
        
        println!("使用时间格式: {}", time_format);
        
        match chrono::NaiveTime::parse_from_str(datetime_str, time_format) {
            Ok(time) => {
                println!("成功解析为时间: {:?}", time);
                let today = Local::now().date_naive();
                let datetime = today.and_time(time);
                let timestamp = DateTime::<Local>::from_naive_utc_and_offset(datetime, *Local::now().offset()).timestamp();
                println!("转换为时间戳: {}", timestamp);
                return Ok(timestamp);
            },
            Err(e) => {
                println!("纯时间解析失败: {:?}", e);
            }
        }
    }
    
    // 3. 如果输入看起来像完整日期时间，尝试使用完整格式解析
    if looks_like_full_datetime {
        println!("输入看起来像完整日期时间，尝试用完整格式解析");
        
        // 使用所有支持的格式
        let possible_formats = get_all_supported_formats();
        
        for possible_format in possible_formats.iter() {
            println!("尝试格式: {}", possible_format);
            
            match NaiveDateTime::parse_from_str(datetime_str, possible_format) {
                Ok(dt) => {
                    println!("成功解析为日期时间: {:?}", dt);
                    let timestamp = DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()).timestamp();
                    println!("转换为时间戳: {}", timestamp);
                    return Ok(timestamp);
                },
                Err(e) => {
                    println!("尝试 {} 解析失败: {:?}", possible_format, e);
                }
            }
        }
    }
    
    // 4. 最后，使用传入的原始格式尝试一次
    println!("前面的尝试都失败了，使用原始格式: {}", format);
    
    match NaiveDateTime::parse_from_str(datetime_str, format) {
        Ok(dt) => {
            println!("成功解析为日期时间: {:?}", dt);
            let timestamp = DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()).timestamp();
            println!("转换为时间戳: {}", timestamp);
            Ok(timestamp)
        },
        Err(e) => {
            println!("使用原始格式解析失败: {:?}", e);
            
            if looks_like_date_only {
                Err(format!("无效的日期格式，请按照选择的格式 \"{}\" 输入，如 \"2023-04-01\"", format))
            } else if looks_like_time_only {
                Err(format!("无效的时间格式，请按照选择的格式 \"{}\" 输入，如 \"15:30:45\"", format))
            } else {
                Err(format!("无效的日期时间格式，请按照选择的格式 \"{}\" 输入，如 \"2023-04-01 15:30:45\"", format))
            }
        }
    }
}

/// 将日期时间字符串转换为毫秒级时间戳，使用指定格式
pub fn datetime_to_ms_timestamp_with_format(datetime_str: &str, format: &str) -> Result<i64, String> {
    println!("尝试将 '{}' 转换为毫秒时间戳，使用格式 '{}'", datetime_str, format);
    
    // 智能判断输入的是日期、时间还是日期时间
    let has_date_chars = datetime_str.contains('-') || datetime_str.contains('/') || 
                        datetime_str.contains('年') || datetime_str.contains('@');
    let has_time_chars = datetime_str.contains(':');
    
    // 检查输入看起来像什么类型
    let looks_like_date_only = has_date_chars && !has_time_chars;
    let looks_like_time_only = !has_date_chars && has_time_chars;
    let looks_like_full_datetime = has_date_chars && has_time_chars;
    
    println!("输入分析: 看起来像日期={}, 看起来像时间={}, 看起来像完整日期时间={}", 
             looks_like_date_only, looks_like_time_only, looks_like_full_datetime);
    
    // 1. 如果输入看起来像纯日期，尝试使用日期格式解析
    if looks_like_date_only {
        println!("输入看起来像纯日期，尝试用日期格式解析");
        
        // 根据分隔符选择合适的格式
        let date_format = if datetime_str.contains('-') {
            "%Y-%m-%d"
        } else if datetime_str.contains('/') {
            "%Y/%m/%d"
        } else if datetime_str.contains('@') {
            "%Y@%m@%d"
        } else if datetime_str.contains('年') {
            "%Y年%m月%d日"
        } else {
            "%Y-%m-%d" // 默认格式
        };
        
        println!("使用日期格式: {}", date_format);
        
        match chrono::NaiveDate::parse_from_str(datetime_str, date_format) {
            Ok(date) => {
                println!("成功解析为日期: {:?}", date);
                let datetime = date.and_hms_opt(0, 0, 0).unwrap();
                let local_dt = DateTime::<Local>::from_naive_utc_and_offset(datetime, *Local::now().offset());
                let ms_timestamp = local_dt.timestamp() * 1000;
                println!("转换为毫秒时间戳: {}", ms_timestamp);
                return Ok(ms_timestamp);
            },
            Err(e) => {
                println!("纯日期解析失败: {:?}", e);
            }
        }
    }
    
    // 2. 如果输入看起来像纯时间，尝试使用时间格式解析
    if looks_like_time_only {
        println!("输入看起来像纯时间，尝试用时间格式解析");
        
        // 根据输入长度选择时间格式
        let time_format = if datetime_str.matches(':').count() == 1 {
            "%H:%M"
        } else {
            "%H:%M:%S"
        };
        
        println!("使用时间格式: {}", time_format);
        
        match chrono::NaiveTime::parse_from_str(datetime_str, time_format) {
            Ok(time) => {
                println!("成功解析为时间: {:?}", time);
                let today = Local::now().date_naive();
                let datetime = today.and_time(time);
                let local_dt = DateTime::<Local>::from_naive_utc_and_offset(datetime, *Local::now().offset());
                let ms_timestamp = local_dt.timestamp() * 1000 + local_dt.timestamp_subsec_millis() as i64;
                println!("转换为毫秒时间戳: {}", ms_timestamp);
                return Ok(ms_timestamp);
            },
            Err(e) => {
                println!("纯时间解析失败: {:?}", e);
            }
        }
    }
    
    // 3. 如果输入看起来像完整日期时间，尝试使用完整格式解析
    if looks_like_full_datetime {
        println!("输入看起来像完整日期时间，尝试用完整格式解析");
        
        // 使用所有支持的格式
        let possible_formats = get_all_supported_formats();
        
        for possible_format in possible_formats.iter() {
            println!("尝试格式: {}", possible_format);
            
            match NaiveDateTime::parse_from_str(datetime_str, possible_format) {
                Ok(dt) => {
                    println!("成功解析为日期时间: {:?}", dt);
                    let local_dt = DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset());
                    let ms_timestamp = local_dt.timestamp() * 1000 + local_dt.timestamp_subsec_millis() as i64;
                    println!("转换为毫秒时间戳: {}", ms_timestamp);
                    return Ok(ms_timestamp);
                },
                Err(e) => {
                    println!("尝试 {} 解析失败: {:?}", possible_format, e);
                }
            }
        }
    }
    
    // 4. 最后，使用传入的原始格式尝试一次
    println!("前面的尝试都失败了，使用原始格式: {}", format);
    
    match NaiveDateTime::parse_from_str(datetime_str, format) {
        Ok(dt) => {
            println!("成功解析为日期时间: {:?}", dt);
            let local_dt = DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset());
            let ms_timestamp = local_dt.timestamp() * 1000 + local_dt.timestamp_subsec_millis() as i64;
            println!("转换为毫秒时间戳: {}", ms_timestamp);
            Ok(ms_timestamp)
        },
        Err(e) => {
            println!("使用原始格式解析失败: {:?}", e);
            
            if looks_like_date_only {
                Err(format!("无效的日期格式，请按照选择的格式 \"{}\" 输入，如 \"2023-04-01\"", format))
            } else if looks_like_time_only {
                Err(format!("无效的时间格式，请按照选择的格式 \"{}\" 输入，如 \"15:30:45\"", format))
            } else {
                Err(format!("无效的日期时间格式，请按照选择的格式 \"{}\" 输入，如 \"2023-04-01 15:30:45\"", format))
            }
        }
    }
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
