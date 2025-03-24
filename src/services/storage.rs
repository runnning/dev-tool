use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new() -> Self {
        let project_dirs = ProjectDirs::from("com", "devtool", "DevTool")
            .expect("无法获取数据目录");
        
        let data_dir = project_dirs.data_dir();
        fs::create_dir_all(data_dir).expect("无法创建数据目录");
        
        Self {
            data_dir: data_dir.to_path_buf(),
        }
    }

    /// 保存历史记录
    pub fn save_history(&self, tool_name: &str, content: &str) -> Result<(), String> {
        let history_file = self.data_dir.join(format!("{}_history.txt", tool_name));
        fs::write(&history_file, content)
            .map_err(|e| format!("保存历史记录失败: {}", e))?;
        Ok(())
    }

    /// 读取历史记录
    pub fn load_history(&self, tool_name: &str) -> Result<String, String> {
        let history_file = self.data_dir.join(format!("{}_history.txt", tool_name));
        fs::read_to_string(&history_file)
            .map_err(|e| format!("读取历史记录失败: {}", e))
    }

    /// 清除历史记录
    pub fn clear_history(&self, tool_name: &str) -> Result<(), String> {
        let history_file = self.data_dir.join(format!("{}_history.txt", tool_name));
        if history_file.exists() {
            fs::remove_file(&history_file)
                .map_err(|e| format!("清除历史记录失败: {}", e))?;
        }
        Ok(())
    }
} 