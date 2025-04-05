use crate::utils::json;
use std::sync::mpsc;

#[derive(Clone, Copy)]
pub struct JsonLogic;

impl JsonLogic {
    pub fn new() -> Self {
        Self
    }

    /// 格式化JSON字符串
    pub fn format(&self, json_str: &str) -> String {
        json::format_json(json_str)
    }

    /// 压缩JSON字符串
    pub fn minify(&self, json_str: &str) -> String {
        json::minify_json(json_str)
    }
    
    /// 异步格式化JSON字符串
    pub fn format_async(&self, json_str: String) -> mpsc::Receiver<String> {
        json::format_json_async(json_str)
    }
    
    /// 异步压缩JSON字符串
    pub fn minify_async(&self, json_str: String) -> mpsc::Receiver<String> {
        json::minify_json_async(json_str)
    }
    
    /// 带进度反馈的异步JSON格式化
    pub fn format_with_progress(&self, json_str: String) -> mpsc::Receiver<json::ProcessUpdate> {
        json::format_json_with_progress(json_str)
    }
    
    /// 带进度反馈的异步JSON压缩
    pub fn minify_with_progress(&self, json_str: String) -> mpsc::Receiver<json::ProcessUpdate> {
        json::minify_json_with_progress(json_str)
    }
} 