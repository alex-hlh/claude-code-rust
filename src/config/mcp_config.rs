//! MCP 服务器配置 - Model Context Protocol 服务器配置管理
//!
//! MCP 是一种让 AI 模型与外部工具和服务交互的协议。
//! 每个 MCP 服务器提供特定的能力（如文件系统、搜索、数据库等）。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// MCP 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// 服务器名称
    pub name: String,
    /// 启动服务器的命令（如 "npx", "python", "./mcp-server"）
    pub command: String,
    /// 命令行参数
    pub args: Vec<String>,
    /// 环境变量
    pub env: std::collections::HashMap<String, String>,
    /// 工作目录（可选）
    pub cwd: Option<PathBuf>,
    /// 服务器当前状态
    pub status: McpServerStatus,
    /// 服务器提供的能力列表
    pub capabilities: Vec<String>,
    /// 是否在启动时自动启动
    pub auto_start: bool,
}

/// MCP 服务器状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum McpServerStatus {
    Running,   // 运行中
    Stopped,   // 已停止
    Error,     // 错误状态
    Unknown,   // 未知状态
    Starting,  // 启动中
}

impl std::fmt::Display for McpServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpServerStatus::Running => write!(f, "running"),
            McpServerStatus::Stopped => write!(f, "stopped"),
            McpServerStatus::Error => write!(f, "error"),
            McpServerStatus::Unknown => write!(f, "unknown"),
            McpServerStatus::Starting => write!(f, "starting"),
        }
    }
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            command: String::new(),
            args: Vec::new(),
            env: std::collections::HashMap::new(),
            cwd: None,
            status: McpServerStatus::Unknown,
            capabilities: Vec::new(),
            auto_start: true,
        }
    }
}

impl McpConfig {
    /// 创建新的 MCP 服务器配置
    pub fn new(name: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            args: Vec::new(),
            env: std::collections::HashMap::new(),
            cwd: None,
            status: McpServerStatus::Unknown,
            capabilities: Vec::new(),
            auto_start: true,
        }
    }

    /// 添加命令行参数
    pub fn with_arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// 添加环境变量
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }
}
