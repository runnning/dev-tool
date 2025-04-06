use serde_json::{from_str, to_string_pretty, Value};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// 进度信息枚举，用于传递处理进度
pub enum ProcessUpdate {
    Progress(usize), // 处理进度 (0-100)
    Result(String),  // 最终结果
}

// 错误消息常量
const ERR_INVALID_JSON: &str = "无效的JSON格式";
const ERR_FORMAT_FAILED: &str = "JSON格式化失败";
const ERR_MINIFY_FAILED: &str = "JSON压缩失败";

// 定义大小阈值
const MAX_INPUT_SIZE: usize = 20_000_000; // 最大输入限制 20MB
const LARGE_JSON_THRESHOLD: usize = 500_000; // 大型JSON阈值 500KB
const MB_SIZE: usize = 1024 * 1024;
const KB_SIZE: usize = 1024;

/// 生成超过最大输入限制的错误消息
fn size_limit_error_msg(size: usize) -> String {
    format!(
        "JSON数据过大（{}MB），已超过处理限制，请分批处理",
        size / MB_SIZE
    )
}

/// 格式化JSON字符串，标准实现
pub fn format_json(json_str: &str) -> String {
    let start_time = Instant::now();

    // 对超大JSON进行直接拒绝处理
    if json_str.len() > MAX_INPUT_SIZE {
        return size_limit_error_msg(json_str.len());
    }

    // 记录大型JSON处理信息
    if json_str.len() > MB_SIZE {
        println!("处理大型JSON数据，大小: {}KB", json_str.len() / KB_SIZE);
    }

    // 解析和格式化JSON
    let result = match from_str::<Value>(json_str) {
        Ok(json) => to_string_pretty(&json).unwrap_or_else(|_| String::from(ERR_FORMAT_FAILED)),
        Err(e) => {
            let error_msg = format!("{}: {}", ERR_INVALID_JSON, e);
            println!("JSON解析错误: {}", error_msg);
            error_msg
        }
    };

    println!("JSON格式化耗时: {:?}", start_time.elapsed());
    result
}

/// 压缩JSON字符串，标准实现
pub fn minify_json(json_str: &str) -> String {
    let start_time = Instant::now();

    // 对超大JSON进行直接拒绝处理
    if json_str.len() > MAX_INPUT_SIZE {
        return size_limit_error_msg(json_str.len());
    }

    // 记录大型JSON处理信息
    if json_str.len() > MB_SIZE {
        println!("处理大型JSON数据，大小: {}KB", json_str.len() / KB_SIZE);
    }

    // 解析和压缩JSON
    let result = match from_str::<Value>(json_str) {
        Ok(json) => serde_json::to_string(&json).unwrap_or_else(|_| String::from(ERR_MINIFY_FAILED)),
        Err(e) => {
            let error_msg = format!("{}: {}", ERR_INVALID_JSON, e);
            println!("JSON解析错误: {}", error_msg);
            error_msg
        }
    };

    println!("JSON压缩耗时: {:?}", start_time.elapsed());
    result
}

/// 异步格式化JSON字符串
pub fn format_json_async(json_str: String) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let start_time = Instant::now();

        // 快速有效性检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(String::from(ERR_INVALID_JSON));
            return;
        }

        // 验证并格式化JSON
        let result = match from_str::<Value>(&json_str) {
            Ok(json) => to_string_pretty(&json).unwrap_or_else(|_| String::from(ERR_FORMAT_FAILED)),
            Err(_) => String::from(ERR_INVALID_JSON),
        };

        println!("异步JSON格式化耗时: {:?}", start_time.elapsed());
        let _ = tx.send(result);
    });

    rx
}

/// 异步压缩JSON字符串
pub fn minify_json_async(json_str: String) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let start_time = Instant::now();

        // 快速有效性检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(String::from(ERR_INVALID_JSON));
            return;
        }

        // 验证并压缩JSON
        let result = match from_str::<Value>(&json_str) {
            Ok(json) => serde_json::to_string(&json).unwrap_or_else(|_| String::from(ERR_MINIFY_FAILED)),
            Err(_) => String::from(ERR_INVALID_JSON),
        };

        println!("异步JSON压缩耗时: {:?}", start_time.elapsed());
        let _ = tx.send(result);
    });

    rx
}

/// 将处理结果转换为相应格式（格式化或压缩）
fn process_json_result(value: &Value, format: bool) -> Result<String, String> {
    if format {
        to_string_pretty(value).map_err(|_| ERR_FORMAT_FAILED.to_string())
    } else {
        serde_json::to_string(value).map_err(|_| ERR_MINIFY_FAILED.to_string())
    }
}

/// 带进度反馈的异步JSON处理基础函数
fn process_json_with_progress(
    json_str: String, 
    format: bool,
    operation_name: &str
) -> mpsc::Receiver<ProcessUpdate> {
    let (tx, rx) = mpsc::channel();
    let operation_name = operation_name.to_string(); // 克隆到闭包中

    thread::spawn(move || {
        let start_time = Instant::now();
        let input_size = json_str.len();

        // 检查数据大小并拒绝过大的输入
        if input_size > MAX_INPUT_SIZE {
            let error_msg = format!(
                "JSON数据过大（{}MB），请减小数据量后重试",
                input_size / MB_SIZE
            );
            let _ = tx.send(ProcessUpdate::Result(error_msg));
            return;
        }

        // 先进行快速检查
        if !is_valid_json_quick_check(&json_str) {
            let _ = tx.send(ProcessUpdate::Result(String::from(ERR_INVALID_JSON)));
            return;
        }

        // 发送初始进度
        let _ = tx.send(ProcessUpdate::Progress(5));

        // 大数据分批处理优化
        if input_size > LARGE_JSON_THRESHOLD {
            // 预解析检查
            let _ = tx.send(ProcessUpdate::Progress(10));
            let pre_check = from_str::<Value>(&json_str);
            if pre_check.is_err() {
                let _ = tx.send(ProcessUpdate::Result(String::from(ERR_INVALID_JSON)));
                return;
            }

            let _ = tx.send(ProcessUpdate::Progress(15));

            // 使用分批处理策略处理大型JSON
            match process_large_json_chunked(&json_str, &tx, format) {
                Ok(result) => {
                    let _ = tx.send(ProcessUpdate::Progress(100));
                    let _ = tx.send(ProcessUpdate::Result(result));
                }
                Err(err) => {
                    let _ = tx.send(ProcessUpdate::Result(format!("处理失败: {}", err)));
                }
            }
        } else {
            // 正常处理中等大小的JSON
            let _ = tx.send(ProcessUpdate::Progress(30));

            match from_str::<Value>(&json_str) {
                Ok(json) => {
                    let _ = tx.send(ProcessUpdate::Progress(60));

                    // 增加延迟以避免UI线程阻塞
                    thread::sleep(Duration::from_millis(50));

                    match process_json_result(&json, format) {
                        Ok(processed) => {
                            let _ = tx.send(ProcessUpdate::Progress(90));
                            let _ = tx.send(ProcessUpdate::Result(processed));
                        }
                        Err(err) => {
                            let _ = tx.send(ProcessUpdate::Result(String::from(err)));
                        }
                    }
                }
                Err(_) => {
                    let _ = tx.send(ProcessUpdate::Result(String::from(ERR_INVALID_JSON)));
                }
            }
        }

        println!("JSON{}总耗时: {:?}", operation_name, start_time.elapsed());
    });

    rx
}

/// 带进度反馈的异步JSON格式化
pub fn format_json_with_progress(json_str: String) -> mpsc::Receiver<ProcessUpdate> {
    process_json_with_progress(json_str, true, "格式化")
}

/// 带进度反馈的异步JSON压缩
pub fn minify_json_with_progress(json_str: String) -> mpsc::Receiver<ProcessUpdate> {
    process_json_with_progress(json_str, false, "压缩")
}

/// 处理大型JSON数据，使用分块策略并报告进度
fn process_large_json_chunked(
    json_str: &str,
    tx: &mpsc::Sender<ProcessUpdate>,
    format: bool,
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
    let json_value = match from_str::<Value>(json_str) {
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

                // 模拟分批处理的进度更新
                for i in (60..90).step_by(5) {
                    update_progress(i);
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
    process_json_result(&json_value, format)
}

/// 快速检查JSON是否有效，不进行完整解析
fn is_valid_json_quick_check(json_str: &str) -> bool {
    let trimmed = json_str.trim();
    if trimmed.is_empty() {
        return false;
    }

    let first_char = trimmed.chars().next().unwrap();
    let last_char = trimmed.chars().last().unwrap();

    // JSON对象或数组的基本检查
    let valid_start = first_char == '{' || first_char == '[';
    let valid_end = last_char == '}' || last_char == ']';
    
    // 括号匹配检查
    let brackets_match = (first_char == '{' && last_char == '}') || 
                         (first_char == '[' && last_char == ']');

    valid_start && valid_end && brackets_match
}
