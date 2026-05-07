//! 应用状态模块 - 管理应用程序的运行时状态
//!
//! AppState 是应用程序的中央状态容器，包含：
//! - 配置设置
//! - 会话历史
//! - 当前对话
//! - 工具注册表
//! - 记忆状态
//!
//! 使用 Arc<RwLock> 实现线程安全的共享状态访问。

use crate::config::Settings;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 应用程序状态 - 整个应用共享的中央状态容器
pub struct AppState {
    /// 配置设置
    pub settings: Settings,
    /// 会话历史记录
    pub session_history: Arc<RwLock<Vec<SessionEntry>>>,
    /// 当前对话
    pub current_conversation: Arc<RwLock<Conversation>>,
    /// 工具注册表状态
    pub tool_registry: Arc<RwLock<ToolRegistryState>>,
    /// 记忆系统状态
    pub memory_state: Arc<RwLock<MemoryState>>,
    /// 运行标志（用于优雅关闭）
    pub running: Arc<RwLock<bool>>,
}

/// 会话条目 - 表示一次独立的会话记录
#[derive(Debug, Clone)]
pub struct SessionEntry {
    /// 会话唯一 ID
    pub id: String,
    /// 会话开始时间
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// 会话结束时间（None 表示仍在进行）
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 消息数量
    pub message_count: usize,
    /// 会话摘要（可选）
    pub summary: Option<String>,
}

/// 当前对话状态
#[derive(Debug, Clone)]
pub struct Conversation {
    /// 对话唯一 ID
    pub id: String,
    /// 对话中的消息列表
    pub messages: Vec<Message>,
    /// 当前使用的模型
    pub model: String,
    /// 已使用的总 token 数
    pub total_tokens: usize,
    /// 已消耗的总费用
    pub total_cost: f64,
}

/// 消息结构 - 对话中的一条消息
#[derive(Debug, Clone)]
pub struct Message {
    /// 消息角色（用户/助手/系统/工具）
    pub role: MessageRole,
    /// 消息内容
    pub content: String,
    /// 时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 工具调用列表（如果有）
    pub tool_calls: Vec<ToolCall>,
}

/// 消息角色枚举
#[derive(Debug, Clone)]
pub enum MessageRole {
    User,      // 用户消息
    Assistant, // AI 助手消息
    System,    // 系统消息
    Tool,      // 工具返回消息
}

/// 工具调用结构
#[derive(Debug, Clone)]
pub struct ToolCall {
    /// 工具名称
    pub name: String,
    /// 工具输入参数（JSON）
    pub input: serde_json::Value,
    /// 工具输出（可选）
    pub output: Option<serde_json::Value>,
    /// 调用状态
    pub status: ToolCallStatus,
}

/// 工具调用状态
#[derive(Debug, Clone)]
pub enum ToolCallStatus {
    Pending,  // 等待执行
    Running,  // 执行中
    Success,  // 执行成功
    Error,    // 执行失败
}

/// 工具注册表状态
#[derive(Debug, Clone)]
pub struct ToolRegistryState {
    /// 已注册的工具列表
    pub tools: Vec<ToolInfo>,
}

/// 工具信息
#[derive(Debug, Clone)]
pub struct ToolInfo {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 是否启用
    pub enabled: bool,
}

/// 记忆状态
#[derive(Debug, Clone, Default)]
pub struct MemoryState {
    /// 已存储的记忆数量
    pub memory_count: usize,
    /// 上次整合时间
    pub last_consolidation: Option<chrono::DateTime<chrono::Utc>>,
    /// 上次整合后的会话数
    pub sessions_since_consolidation: usize,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            session_history: Arc::new(RwLock::new(Vec::new())),
            current_conversation: Arc::new(RwLock::new(Conversation::new())),
            tool_registry: Arc::new(RwLock::new(ToolRegistryState::default())),
            memory_state: Arc::new(RwLock::new(MemoryState::default())),
            running: Arc::new(RwLock::new(true)),
        }
    }

    /// 向当前对话添加消息
    pub async fn add_message(&self, role: MessageRole, content: String) {
        let mut conversation = self.current_conversation.write().await;
        conversation.messages.push(Message {
            role,
            content,
            timestamp: chrono::Utc::now(),
            tool_calls: Vec::new(),
        });
    }

    /// 获取当前对话的所有消息
    pub async fn get_messages(&self) -> Vec<Message> {
        let conversation = self.current_conversation.read().await;
        conversation.messages.clone()
    }

    /// 清空当前对话
    pub async fn clear_conversation(&self) {
        let mut conversation = self.current_conversation.write().await;
        conversation.messages.clear();
        conversation.total_tokens = 0;
        conversation.total_cost = 0.0;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(Settings::default())
    }
}

impl Conversation {
    /// 创建新对话
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            messages: Vec::new(),
            model: "sonnet".to_string(),
            total_tokens: 0,
            total_cost: 0.0,
        }
    }

    /// 获取消息数量
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

impl Default for Conversation {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ToolRegistryState {
    fn default() -> Self {
        Self {
            tools: vec![
                ToolInfo {
                    name: "file_read".to_string(),
                    description: "读取文件内容".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "file_edit".to_string(),
                    description: "编辑文件内容".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "file_write".to_string(),
                    description: "写入新文件".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "execute_command".to_string(),
                    description: "执行 Shell 命令".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "search".to_string(),
                    description: "在文件中搜索模式".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "list_files".to_string(),
                    description: "列出目录内容".to_string(),
                    enabled: true,
                },
            ],
        }
    }
}
