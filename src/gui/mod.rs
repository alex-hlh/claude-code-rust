//! GUI 模块 - 使用 egui/eframe 的桌面 GUI
//!
//! 本模块为 Claude Code 提供原生桌面 GUI 界面，采用现代化的响应式设计。

pub mod app;           // 主应用
pub mod chat;          // 聊天界面
pub mod sidebar;       // 侧边栏导航
pub mod settings;      // 设置面板
pub mod theme;         // 主题系统
pub mod syntax_highlight;  // 语法高亮
pub mod tool_calls;    // 工具调用展示

pub use app::ClaudeCodeApp;
pub use theme::Theme;

/// Async message type for GUI communication
#[derive(Debug, Clone)]
pub enum GuiMessage {
    /// Send a chat message to the API
    SendMessage { messages: Vec<crate::api::ChatMessage> },
    /// Received a chunk of streaming response
    StreamChunk { content: String, done: bool },
    /// API error occurred
    ApiError { error: String },
    /// Test connection result
    TestConnectionResult { success: bool, message: String },
    /// Settings updated
    SettingsUpdated,
}
