use crate::utils;

#[derive(Clone, Copy)]
pub struct JsonLogic;

impl JsonLogic {
    /// 格式化JSON字符串
    pub fn format(&self, input: &str) -> String {
        utils::json::format_json(input)
    }

    /// 压缩JSON字符串
    pub fn minify(&self, input: &str) -> String {
        utils::json::minify_json(input)
    }
} 