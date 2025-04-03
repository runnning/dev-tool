use serde_json::{from_str, to_string_pretty, Value};
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

/// 格式化JSON字符串，标准实现
pub fn format_json(json_str: &str) -> String {
    // 记录开始时间，用于性能监控
    let start_time = Instant::now();
    
    // 使用直接的JSON解析和格式化方式
    let result = match from_str::<Value>(json_str) {
        Ok(json) => to_string_pretty(&json).unwrap_or_else(|_| String::from("JSON格式化失败")),
        Err(_) => String::from("无效的JSON格式"),
    };
    
    // 计算处理耗时
    let elapsed = start_time.elapsed();
    println!("JSON格式化耗时: {:?}", elapsed);
    
    result
}

/// 压缩JSON字符串，标准实现
pub fn minify_json(json_str: &str) -> String {
    // 记录开始时间，用于性能监控
    let start_time = Instant::now();
    
    // 使用直接的JSON解析和压缩方式
    let result = match from_str::<Value>(json_str) {
        Ok(json) => serde_json::to_string(&json).unwrap_or_else(|_| String::from("JSON压缩失败")),
        Err(_) => String::from("无效的JSON格式"),
    };
    
    // 计算处理耗时
    let elapsed = start_time.elapsed();
    println!("JSON压缩耗时: {:?}", elapsed);
    
    result
}

/// 异步格式化JSON字符串，返回一个接收处理结果的通道
pub fn format_json_async(json_str: String) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        // 记录开始时间，用于性能监控
        let start_time = Instant::now();
        
        // 尝试在不进行完整解析的情况下进行快速有效性检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(String::from("无效的JSON格式"));
            return;
        }
        
        // 首先验证JSON格式是否有效并处理
        let result = match serde_json::from_str::<Value>(&json_str) {
            Ok(json) => {
                match serde_json::to_string_pretty(&json) {
                    Ok(formatted) => formatted,
                    Err(_) => String::from("JSON格式化失败")
                }
            },
            Err(_) => String::from("无效的JSON格式"),
        };
        
        // 计算处理耗时
        let elapsed = start_time.elapsed();
        println!("异步JSON格式化耗时: {:?}", elapsed);
        
        let _ = tx.send(result);
    });
    
    rx
}

/// 异步压缩JSON字符串，返回一个接收处理结果的通道
pub fn minify_json_async(json_str: String) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        // 记录开始时间，用于性能监控
        let start_time = Instant::now();
        
        // 尝试在不进行完整解析的情况下进行快速有效性检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(String::from("无效的JSON格式"));
            return;
        }
        
        // 首先验证JSON格式是否有效并处理
        let result = match serde_json::from_str::<Value>(&json_str) {
            Ok(json) => {
                match serde_json::to_string(&json) {
                    Ok(minified) => minified,
                    Err(_) => String::from("JSON压缩失败")
                }
            },
            Err(_) => String::from("无效的JSON格式"),
        };
        
        // 计算处理耗时
        let elapsed = start_time.elapsed();
        println!("异步JSON压缩耗时: {:?}", elapsed);
        
        let _ = tx.send(result);
    });
    
    rx
}

/// 快速检查JSON是否有效，不进行完整解析
/// 这个函数检查基本的JSON结构，如括号匹配等
fn is_valid_json_quick_check(json_str: &str) -> bool {
    // 简单的检查：是否以{ 或 [ 开始，以} 或 ]结束
    let trimmed = json_str.trim();
    if trimmed.is_empty() {
        return false;
    }
    
    let first_char = trimmed.chars().next().unwrap();
    let last_char = trimmed.chars().last().unwrap();
    
    // JSON对象或数组的检查
    let valid_start = first_char == '{' || first_char == '[';
    let valid_end = last_char == '}' || last_char == ']';
    
    // 检查括号是否匹配
    if (first_char == '{' && last_char != '}') || (first_char == '[' && last_char != ']') {
        return false;
    }
    
    valid_start && valid_end
}
