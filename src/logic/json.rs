use crate::utils::json;

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
} 