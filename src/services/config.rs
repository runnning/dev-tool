use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: String,
    pub language: String,
    pub time_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "zh-CN".to_string(),
            time_format: "YYYY-MM-DD HH:mm:ss".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ConfigService {
    config_path: PathBuf,
}

impl ConfigService {
    pub fn new() -> Self {
        let project_dirs = ProjectDirs::from("com", "devtool", "DevTool")
            .expect("无法获取配置目录");
        
        let config_dir = project_dirs.config_dir();
        fs::create_dir_all(config_dir).expect("无法创建配置目录");
        
        Self {
            config_path: config_dir.join("config.json"),
        }
    }

    pub fn load(&self) -> Config {
        if let Ok(content) = fs::read_to_string(&self.config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        let default_config = Config::default();
        self.save(&default_config).ok();
        default_config
    }

    pub fn save(&self, config: &Config) -> Result<(), String> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        
        fs::write(&self.config_path, content)
            .map_err(|e| format!("保存配置失败: {}", e))?;
        
        Ok(())
    }
} 