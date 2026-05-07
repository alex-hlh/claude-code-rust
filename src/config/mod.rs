//! 配置模块 - 管理应用程序的所有配置
//!
//! 包含以下子模块：
//! - api_config: API 配置（API密钥、URL、模型等）
//! - mcp_config: MCP 服务器配置
//!
//! 配置文件的默认存储路径：~/.claude-code/settings.json

pub mod api_config;   // API 配置
pub mod mcp_config;   // MCP 配置

pub use api_config::ApiConfig;
pub use mcp_config::{McpConfig, McpServerStatus};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 全局设置结构体 - 包含应用程序的所有配置
///
/// # 示例
/// ```
/// let settings = Settings::load()?;
/// println!("使用模型: {}", settings.model);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// API 配置（密钥、URL、模型、超时等）
    pub api: ApiConfig,
    /// MCP 服务器配置列表
    pub mcp_servers: Vec<McpConfig>,
    /// 选择的模型（如 "sonnet", "claude-3-5-sonnet"）
    pub model: String,
    /// 是否启用详细日志
    pub verbose: bool,
    /// 工作目录
    pub working_dir: PathBuf,
    /// 记忆系统设置
    pub memory: MemorySettings,
    /// 语音输入设置
    pub voice: VoiceSettings,
    /// 插件系统设置
    pub plugins: PluginSettings,
}

/// 记忆系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySettings {
    /// 是否启用记忆持久化
    pub enabled: bool,
    /// 记忆文件存储路径
    pub path: PathBuf,
    /// 自动整合间隔（小时）
    pub consolidation_interval: u64,
    /// 最大记忆条数
    pub max_memories: usize,
}

/// 语音输入配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// 是否启用语音输入
    pub enabled: bool,
    /// 按住说话模式（vs 连续监听）
    pub push_to_talk: bool,
    /// 静音检测阈值（0.0-1.0）
    pub silence_threshold: f32,
    /// 音频采样率（Hz）
    pub sample_rate: u32,
}

/// 插件系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSettings {
    /// 是否启用插件系统
    pub enabled: bool,
    /// 插件目录路径
    pub plugin_dir: PathBuf,
    /// 是否自动更新插件
    pub auto_update: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let config_dir = home.join(".claude-code");

        Self {
            api: ApiConfig::default(),
            mcp_servers: Vec::new(),
            model: "sonnet".to_string(),
            verbose: false,
            working_dir: PathBuf::from("."),
            memory: MemorySettings {
                enabled: true,
                path: config_dir.join("memory.json"),
                consolidation_interval: 24,
                max_memories: 1000,
            },
            voice: VoiceSettings {
                enabled: false,
                push_to_talk: false,
                silence_threshold: 0.01,
                sample_rate: 16000,
            },
            plugins: PluginSettings {
                enabled: true,
                plugin_dir: config_dir.join("plugins"),
                auto_update: true,
            },
        }
    }
}

impl Settings {
    /// 从文件加载设置
    ///
    /// 尝试从 ~/.claude-code/settings.json 读取配置。
    /// 如果文件不存在，则创建默认配置并保存。
    pub fn load() -> anyhow::Result<Self> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let config_path = home.join(".claude-code").join("settings.json");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let settings: Settings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            let settings = Settings::default();
            settings.save()?;
            Ok(settings)
        }
    }

    /// 将当前设置保存到文件
    pub fn save(&self) -> anyhow::Result<()> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let config_dir = home.join(".claude-code");
        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("settings.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }

    /// 设置配置项的值
    ///
    /// 支持的配置项：
    /// - `model`: 模型名称
    /// - `verbose`: 详细日志
    /// - `api_key`: API 密钥
    /// - `base_url`: API 基础 URL
    /// - `max_tokens`: 最大 token 数
    /// - `timeout`: 超时时间（秒）
    /// - `streaming`: 是否启用流式响应
    /// - `memory.enabled`: 是否启用记忆
    /// - `voice.enabled`: 是否启用语音
    pub fn set(key: &str, value: &str) -> anyhow::Result<()> {
        let mut settings = Self::load()?;

        match key {
            "model" => settings.model = value.to_string(),
            "verbose" => settings.verbose = value.parse().unwrap_or(false),
            "api_key" => settings.api.api_key = Some(value.to_string()),
            "base_url" => settings.api.base_url = value.to_string(),
            "max_tokens" => settings.api.max_tokens = value.parse().unwrap_or(4096),
            "timeout" => settings.api.timeout = value.parse().unwrap_or(120),
            "streaming" => settings.api.streaming = value.parse().unwrap_or(true),
            "memory.enabled" => settings.memory.enabled = value.parse().unwrap_or(true),
            "voice.enabled" => settings.voice.enabled = value.parse().unwrap_or(false),
            _ => return Err(anyhow::anyhow!("未知配置项: {}", key)),
        }

        settings.save()?;
        Ok(())
    }

    /// 重置设置为默认值
    pub fn reset() -> anyhow::Result<()> {
        let settings = Settings::default();
        settings.save()?;
        Ok(())
    }
}
