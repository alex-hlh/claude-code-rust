//! Claude Code Rust - 核心库定义
//!
//! Rust 完整实现的 Claude Code CLI，包含以下核心功能：
//! - 异步优先架构（Tokio）
//! - 原生终端 UI（Ratatui）
//! - MCP 协议支持
//! - 语音输入支持
//! - 记忆管理与团队同步
//! - 插件系统
//! - SSH 连接支持
//! - 远程执行
//! - 项目初始化
//! - WebAssembly 浏览器环境支持
//! - 原生 GUI（egui/eframe）
//! - 插件市场 Web 界面
//! - 多语言国际化支持（i18n）

// ========================
// 核心模块定义
// ========================

pub mod cli;          // 命令行参数解析与子命令分发
pub mod tools;        // 工具注册表与内置工具（文件读写、搜索、执行命令等）
pub mod api;          // API 客户端（OpenAI/DeepSeek 兼容）
pub mod config;       // 配置管理（API配置、MCP配置、全局设置）
pub mod state;        // 应用状态管理（会话历史、当前对话、工具注册表）
pub mod mcp;          // MCP（Model Context Protocol）协议实现
pub mod voice;        // 语音输入处理
pub mod memory;       // 记忆管理系统（会话、历史、上下文、持久化、整合）
pub mod plugins;      // 插件系统（加载、注册、隔离、钩子）
pub mod utils;        // 工具函数（项目初始化等）
pub mod services;     // 服务层（Agent、AutoDream、MagicDocs、语音等）
pub mod session;      // 会话管理
pub mod terminal;     // 终端界面
pub mod advanced;     // 高级功能（SSH、远程执行、项目初始化）
pub mod skills;       // Skills 技能系统

// ========================
// 特性门控模块
// 仅在对应 feature 启用时编译
// ========================

#[cfg(feature = "wasm")]
pub mod wasm;         // WebAssembly 支持（浏览器环境）

#[cfg(feature = "gui-egui")]
pub mod gui;          // 原生 GUI（egui/eframe）

#[cfg(feature = "web")]
pub mod web;         // Web 服务器（插件市场 Web 界面）

#[cfg(feature = "i18n")]
pub mod i18n;        // 国际化支持

// ========================
// 公共 API 导出
// ========================

pub use cli::Cli;                             // CLI 主入口
pub use state::AppState;                      // 应用状态
pub use tools::ToolRegistry;                  // 工具注册表
pub use api::{ApiClient, AnthropicClient, ChatMessage};  // API 客户端与消息类型
pub use config::Settings;                     // 全局设置
pub use mcp::McpManager;                      // MCP 管理器
pub use voice::VoiceInput;                    // 语音输入
pub use memory::MemoryManager;                // 记忆管理器
pub use plugins::PluginManager;               // 插件管理器
pub use skills::{Skill, SkillRegistry, SkillExecutor, SkillContext, SkillParams, SkillResult, SkillError, SkillCategory};  // 技能系统

// ========================
// 特性门控导出
// ========================

#[cfg(feature = "wasm")]
pub use wasm::ClaudeCodeWasm;                 // WASM 导出

#[cfg(feature = "gui-egui")]
pub use gui::ClaudeCodeApp;                   // GUI 应用导出

#[cfg(feature = "web")]
pub use web::WebServer;                      // Web 服务器导出

#[cfg(feature = "i18n")]
pub use i18n::Translator;                    // 翻译器导出
