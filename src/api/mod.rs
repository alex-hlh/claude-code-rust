//! API 模块 - OpenAI/DeepSeek 兼容的 API 客户端
//!
//! 提供与 LLM API 通信的客户端实现，支持：
//! - 单次对话请求
//! - 流式响应
//! - 工具调用（Tool Use）
//! - 多角色消息构建

use crate::config::Settings;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// API 客户端 - 与 LLM API 通信的主接口
#[derive(Clone)]
pub struct ApiClient {
    settings: Settings,
    http_client: std::sync::Arc<Client>,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new(settings: Settings) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(settings.api.timeout))
            .build()
            .unwrap_or_default();

        Self {
            settings,
            http_client: std::sync::Arc::new(http_client),
        }
    }

    /// 获取 API 密钥
    pub fn get_api_key(&self) -> Option<String> {
        self.settings.api.get_api_key()
    }

    /// 获取基础 URL
    pub fn get_base_url(&self) -> String {
        self.settings.api.get_base_url()
    }

    /// 获取当前使用的模型
    pub fn get_model(&self) -> &str {
        &self.settings.model
    }

    /// 发送对话请求（非流式）
    ///
    /// # 参数
    /// - `messages`: 消息列表
    /// - `tools`: 工具定义列表（可选）
    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<Vec<ToolDefinition>>,
    ) -> anyhow::Result<ChatResponse> {
        let api_key = self
            .get_api_key()
            .ok_or_else(|| anyhow::anyhow!("API 密钥未配置"))?;

        let request = ChatRequest {
            model: self.settings.api.get_model_id(&self.settings.model),
            messages,
            max_tokens: self.settings.api.max_tokens,
            stream: false,
            temperature: 0.7,
            tools,
        };

        let url = format!("{}/v1/chat/completions", self.get_base_url());

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API 错误 ({}): {}", status, body));
        }

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    /// 发送流式对话请求
    ///
    /// 返回原始 HTTP 响应，需要调用者自己解析流式数据
    pub async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        tools: Option<Vec<ToolDefinition>>,
    ) -> anyhow::Result<reqwest::Response> {
        let api_key = self
            .get_api_key()
            .ok_or_else(|| anyhow::anyhow!("API 密钥未配置"))?;

        let request = ChatRequest {
            model: self.settings.api.get_model_id(&self.settings.model),
            messages,
            max_tokens: self.settings.api.max_tokens,
            stream: true,
            temperature: 0.7,
            tools,
        };

        let url = format!("{}/v1/chat/completions", self.get_base_url());

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        Ok(response)
    }
}

/// 工具定义 - 描述一个可调用工具的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub r#type: String,
    pub function: ToolFunction,
}

impl ToolDefinition {
    /// 创建新的工具定义
    pub fn new(name: impl Into<String>, description: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            r#type: "function".to_string(),
            function: ToolFunction {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }
}

/// 工具函数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// 工具调用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

/// 工具调用函数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

/// 对话消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    /// 创建用户消息
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// 创建助手消息
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// 创建带工具调用的助手消息
    pub fn assistant_with_tools(tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: None,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        }
    }

    /// 创建系统消息
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// 创建工具返回消息
    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: "tool".to_string(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }
}

/// API 请求结构体
#[derive(Debug, Clone, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: usize,
    stream: bool,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolDefinition>>,
}

/// API 响应结构
#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

/// 选择项
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

/// Token 使用量
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// 流式响应数据块
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

/// 流式选择项
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    pub index: i32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

/// 流式增量数据
#[derive(Debug, Clone, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}

/// 类型别名：AnthropicClient 等同于 ApiClient
pub type AnthropicClient = ApiClient;
