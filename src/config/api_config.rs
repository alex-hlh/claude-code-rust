//! API 配置 - 管理与 LLM API 的连接配置
//!
//! 支持多种 API 提供商：
//! - Anthropic API（Claude）
//! - DeepSeek API
//! - DashScope API
//!
//! API 密钥可以通过环境变量或配置文件设置。

use serde::{Deserialize, Serialize};

/// API 配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// API 密钥（可选，也支持通过环境变量设置）
    pub api_key: Option<String>,
    /// API 请求的基础 URL
    pub base_url: String,
    /// 单次请求的最大 token 数
    pub max_tokens: usize,
    /// 请求超时时间（秒）
    pub timeout: u64,
    /// 是否启用流式响应
    pub streaming: bool,
    /// 需要包含的 Beta 请求头
    pub beta_headers: Vec<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            // 按优先级尝试读取环境变量
            api_key: std::env::var("ANTHROPIC_API_KEY").ok()
                .or(std::env::var("DASHSCOPE_API_KEY").ok())
                .or(std::env::var("DEEPSEEK_API_KEY").ok()),
            // 默认使用 Anthropic API，也可以通过环境变量覆盖
            base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "https://api.anthropic.com".to_string()),
            max_tokens: 4096,
            timeout: 120,
            streaming: true,
            beta_headers: vec![],
        }
    }
}

impl ApiConfig {
    /// 获取 API 密钥，优先检查环境变量
    pub fn get_api_key(&self) -> Option<String> {
        std::env::var("ANTHROPIC_API_KEY").ok()
            .or(std::env::var("DASHSCOPE_API_KEY").ok())
            .or(std::env::var("DEEPSEEK_API_KEY").ok())
            .or(self.api_key.clone())
    }

    /// 获取基础 URL，优先检查环境变量
    pub fn get_base_url(&self) -> String {
        std::env::var("API_BASE_URL")
            .unwrap_or_else(|_| self.base_url.clone())
    }

    /// 根据模型名称获取完整的模型 ID
    ///
    /// 支持的模型别名：
    /// - "opus" -> claude-3-opus-20240229
    /// - "sonnet" -> claude-3-5-sonnet-20241022
    /// - "haiku" -> claude-3-5-haiku-20241022
    pub fn get_model_id(&self, model: &str) -> String {
        match model {
            "opus" => "claude-3-opus-20240229".to_string(),
            "sonnet" => "claude-3-5-sonnet-20241022".to_string(),
            "haiku" => "claude-3-5-haiku-20241022".to_string(),
            // 未知模型直接返回原名
            _ => model.to_string(),
        }
    }
}
