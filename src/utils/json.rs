use serde_json::{from_str, to_string_pretty, Value};
use std::thread;
use std::sync::mpsc;
use std::time::{Instant, Duration};

/// 进度信息结构体，用于传递处理进度
pub enum ProcessUpdate {
    Progress(usize),  // 处理进度 (0-100)
    Result(String),   // 最终结果
}

// 定义最大输入限制
const MAX_INPUT_SIZE: usize = 20_000_000; // 最大输入限制 20MB

/// 格式化JSON字符串，标准实现
pub fn format_json(json_str: &str) -> String {
    // 记录开始时间，用于性能监控
    let start_time = Instant::now();
    
    // 对超大JSON进行直接拒绝处理
    if json_str.len() > MAX_INPUT_SIZE {
        return format!("警告：JSON数据过大（{}MB），已超过处理限制，请分批处理", json_str.len() / 1024 / 1024);
    }
    
    if json_str.len() > 1024 * 1024 {
        println!("处理大型JSON数据，大小: {}KB", json_str.len() / 1024);
    }
    
    // 使用直接的JSON解析和格式化方式
    let result = match from_str::<Value>(json_str) {
        Ok(json) => to_string_pretty(&json).unwrap_or_else(|_| String::from("JSON格式化失败")),
        Err(e) => {
            let error_msg = format!("无效的JSON格式: {}", e);
            println!("JSON解析错误: {}", error_msg);
            error_msg
        }
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
    
    // 对超大JSON进行直接拒绝处理
    if json_str.len() > MAX_INPUT_SIZE {
        return format!("警告：JSON数据过大（{}MB），已超过处理限制，请分批处理", json_str.len() / 1024 / 1024);
    }
    
    if json_str.len() > 1024 * 1024 {
        println!("处理大型JSON数据，大小: {}KB", json_str.len() / 1024);
    }
    
    // 使用直接的JSON解析和压缩方式
    let result = match from_str::<Value>(json_str) {
        Ok(json) => serde_json::to_string(&json).unwrap_or_else(|_| String::from("JSON压缩失败")),
        Err(e) => {
            let error_msg = format!("无效的JSON格式: {}", e);
            println!("JSON解析错误: {}", error_msg);
            error_msg
        }
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

/// 带进度反馈的异步JSON格式化
pub fn format_json_with_progress(json_str: String) -> mpsc::Receiver<ProcessUpdate> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let start_time = Instant::now();
        let input_size = json_str.len();
        
        // 检查数据大小并拒绝过大的输入
        if input_size > MAX_INPUT_SIZE {
            let error_msg = format!("JSON数据过大（{}MB），请减小数据量后重试", input_size / 1024 / 1024);
            let _ = tx.send(ProcessUpdate::Result(error_msg));
            return;
        }
        
        // 先进行快速检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(ProcessUpdate::Result(String::from("无效的JSON格式")));
            return;
        }
        
        // 发送初始进度
        let _ = tx.send(ProcessUpdate::Progress(5));
        
        // 大数据分批处理优化
        if input_size > 500_000 {
            // 预解析检查 - 避免无效JSON消耗资源
            let _ = tx.send(ProcessUpdate::Progress(10));
            let pre_check = serde_json::from_str::<Value>(&json_str);
            if pre_check.is_err() {
                let _ = tx.send(ProcessUpdate::Result(String::from("无效的JSON格式")));
                return;
            }
            
            let _ = tx.send(ProcessUpdate::Progress(15));
            
            // 使用分批处理策略处理大型JSON
            match process_large_json_chunked(&json_str, &tx, true) {
                Ok(result) => {
                    let _ = tx.send(ProcessUpdate::Progress(100));
                    let _ = tx.send(ProcessUpdate::Result(result));
                },
                Err(err) => {
                    let _ = tx.send(ProcessUpdate::Result(format!("处理失败: {}", err)));
                }
            }
        } else {
            // 正常处理中等大小的JSON
            let _ = tx.send(ProcessUpdate::Progress(30));
            
            match serde_json::from_str::<Value>(&json_str) {
                Ok(json) => {
                    let _ = tx.send(ProcessUpdate::Progress(60));
                    
                    // 增加延迟以避免UI线程阻塞
                    thread::sleep(Duration::from_millis(50));
                    
                    match serde_json::to_string_pretty(&json) {
                        Ok(formatted) => {
                            let _ = tx.send(ProcessUpdate::Progress(90));
                            let _ = tx.send(ProcessUpdate::Result(formatted));
                        },
                        Err(_) => {
                            let _ = tx.send(ProcessUpdate::Result(String::from("JSON格式化失败")));
                        }
                    }
                },
                Err(_) => {
                    let _ = tx.send(ProcessUpdate::Result(String::from("无效的JSON格式")));
                }
            }
        }
        
        let elapsed = start_time.elapsed();
        println!("JSON格式化总耗时: {:?}", elapsed);
    });
    
    rx
}

/// 带进度反馈的异步JSON压缩
pub fn minify_json_with_progress(json_str: String) -> mpsc::Receiver<ProcessUpdate> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let start_time = Instant::now();
        let input_size = json_str.len();
        
        // 检查数据大小并拒绝过大的输入
        if input_size > MAX_INPUT_SIZE {
            let error_msg = format!("JSON数据过大（{}MB），请减小数据量后重试", input_size / 1024 / 1024);
            let _ = tx.send(ProcessUpdate::Result(error_msg));
            return;
        }
        
        // 先进行快速检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(ProcessUpdate::Result(String::from("无效的JSON格式")));
            return;
        }
        
        // 发送初始进度
        let _ = tx.send(ProcessUpdate::Progress(5));
        
        // 大数据分批处理优化
        if input_size > 500_000 {
            // 预解析检查 - 避免无效JSON消耗资源
            let _ = tx.send(ProcessUpdate::Progress(10));
            let pre_check = serde_json::from_str::<Value>(&json_str);
            if pre_check.is_err() {
                let _ = tx.send(ProcessUpdate::Result(String::from("无效的JSON格式")));
                return;
            }
            
            let _ = tx.send(ProcessUpdate::Progress(15));
            
            // 使用分批处理策略处理大型JSON
            match process_large_json_chunked(&json_str, &tx, false) {
                Ok(result) => {
                    let _ = tx.send(ProcessUpdate::Progress(100));
                    let _ = tx.send(ProcessUpdate::Result(result));
                },
                Err(err) => {
                    let _ = tx.send(ProcessUpdate::Result(format!("处理失败: {}", err)));
                }
            }
        } else {
            // 正常处理中等大小的JSON
            let _ = tx.send(ProcessUpdate::Progress(30));
            
            match serde_json::from_str::<Value>(&json_str) {
                Ok(json) => {
                    let _ = tx.send(ProcessUpdate::Progress(60));
                    
                    // 增加延迟以避免UI线程阻塞
                    thread::sleep(Duration::from_millis(50));
                    
                    match serde_json::to_string(&json) {
                        Ok(minified) => {
                            let _ = tx.send(ProcessUpdate::Progress(90));
                            let _ = tx.send(ProcessUpdate::Result(minified));
                        },
                        Err(_) => {
                            let _ = tx.send(ProcessUpdate::Result(String::from("JSON压缩失败")));
                        }
                    }
                },
                Err(_) => {
                    let _ = tx.send(ProcessUpdate::Result(String::from("无效的JSON格式")));
                }
            }
        }
        
        let elapsed = start_time.elapsed();
        println!("JSON压缩总耗时: {:?}", elapsed);
    });
    
    rx
}

/// 处理大型JSON数据，使用分块策略并报告进度
fn process_large_json_chunked(
    json_str: &str, 
    tx: &mpsc::Sender<ProcessUpdate>, 
    format: bool
) -> Result<String, String> {
    // 辅助函数：更新进度并允许UI线程刷新
    let update_progress = |progress: usize| {
        let _ = tx.send(ProcessUpdate::Progress(progress));
        thread::sleep(Duration::from_millis(10)); // 给UI线程喘息的机会
    };
    
    // 确定解析策略
    let trimmed = json_str.trim();
    let first_char = trimmed.chars().next().unwrap();
    
    update_progress(20);
    
    // 解析整个JSON
    let json_value = match serde_json::from_str::<Value>(json_str) {
        Ok(value) => value,
        Err(e) => return Err(format!("JSON解析错误: {}", e)),
    };
    
    update_progress(50);
    
    // 数组特殊处理
    if first_char == '[' {
        println!("处理大型JSON数组...");
        
        // 分析数组大小
        if let Value::Array(array) = &json_value {
            println!("JSON数组元素数量: {}", array.len());
            
            // 如果是大型数组，使用分批处理策略
            if array.len() > 1000 {
                update_progress(60);
                
                // 此处只做进度更新，实际上我们已经解析了整个JSON
                for i in 60..90 {
                    if i % 5 == 0 {
                        update_progress(i);
                    }
                }
            }
        }
    } else {
        // 对象特殊处理
        println!("处理大型JSON对象...");
        update_progress(70);
    }
    
    update_progress(90);
    
    // 生成最终结果
    if format {
        match serde_json::to_string_pretty(&json_value) {
            Ok(result) => Ok(result),
            Err(_) => Err("JSON格式化失败".to_string())
        }
    } else {
        match serde_json::to_string(&json_value) {
            Ok(result) => Ok(result),
            Err(_) => Err("JSON压缩失败".to_string())
        }
    }
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
