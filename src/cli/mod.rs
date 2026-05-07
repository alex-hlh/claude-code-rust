//! CLI 模块 - 命令行接口定义
//!
//! 使用 Clap 库定义所有命令行参数和子命令。

pub mod args;     // CLI 参数实现
pub mod commands; // 命令定义
pub mod repl;     // REPL 实现
pub mod ui;       // UI 工具

pub use args::Cli;
pub use repl::Repl;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Claude Code - AI 驱动的编程助手
#[derive(Parser, Debug)]
#[command(name = "claude-code")]
#[command(author = "Anthropic")]
#[command(version = "0.1.0")]
#[command(about = "Claude Code 的高性能 Rust 实现")]
#[command(disable_version_flag = true)]
#[command(disable_help_subcommand = true)]
pub struct CliArgs {
    /// 项目目录路径
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// 使用的模型（sonnet, opus, haiku）
    #[arg(short, long, default_value = "sonnet")]
    pub model: String,

    /// 启用详细日志
    #[arg(short, long)]
    pub verbose: bool,

    /// 以非交互模式运行
    #[arg(short, long)]
    pub no_interactive: bool,

    /// 打印版本信息
    #[arg(long)]
    pub version: bool,

    /// 打印系统信息
    #[arg(long)]
    pub info: bool,

    /// 子命令
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// CLI 子命令枚举
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 启动交互式 REPL 会话
    Repl {
        /// 初始提示
        #[arg(short, long)]
        prompt: Option<String>,
    },

    /// 执行单次查询
    Query {
        /// 要执行的查询
        #[arg(short, long)]
        prompt: String,
    },

    /// 管理配置设置
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },

    /// 管理 MCP 服务器
    Mcp {
        #[command(subcommand)]
        action: McpCommands,
    },

    /// 管理插件
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },

    /// 管理记忆和会话
    Memory {
        #[command(subcommand)]
        action: MemoryCommands,
    },

    /// 语音输入模式
    Voice {
        /// 启用按住说话模式
        #[arg(short, long)]
        push_to_talk: bool,
    },

    /// 初始化新项目
    Init {
        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,
    },

    /// 更新到最新版本
    Update,

    /// 显示帮助信息
    Help {
        /// 要显示帮助的主题
        #[arg(short, long)]
        topic: Option<String>,
    },

    /// 管理后台服务
    Services {
        #[command(subcommand)]
        action: ServiceCommands,
    },

    /// 运行 Agent
    Agent {
        /// Agent 类型（guide, explore, plan, verify, general）
        #[arg(short, long)]
        agent_type: String,
        /// 给 Agent 的提示
        #[arg(short, long)]
        prompt: String,
    },

    /// 管理 Magic Docs
    MagicDocs {
        #[command(subcommand)]
        action: MagicDocsCommands,
    },

    /// 团队记忆同步
    TeamSync {
        #[command(subcommand)]
        action: TeamSyncCommands,
    },

    /// 管理 Skills
    Skills {
        #[command(subcommand)]
        action: SkillsCommands,
    },

    /// 运行压力测试
    StressTest {
        /// 并发请求数
        #[arg(short, long, default_value = "5")]
        concurrency: usize,
        /// 每个请求的迭代次数
        #[arg(short, long, default_value = "10")]
        iterations: usize,
    },
}

/// 配置子命令
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// 显示当前配置
    Show,

    /// 设置配置值
    Set {
        /// 配置键
        key: String,
        /// 配置值
        value: String,
    },

    /// 重置配置为默认值
    Reset,
}

/// MCP 子命令
#[derive(Subcommand, Debug)]
pub enum McpCommands {
    /// 列出配置的 MCP 服务器
    List,

    /// 添加新的 MCP 服务器
    Add {
        /// 服务器名称
        name: String,
        /// 服务器命令
        command: String,
    },

    /// 移除 MCP 服务器
    Remove {
        /// 服务器名称
        name: String,
    },

    /// 重启 MCP 服务器
    Restart {
        /// 服务器名称
        name: String,
    },
}

/// 插件子命令
#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// 列出已安装的插件
    List,

    /// 安装插件
    Install {
        /// 插件名称或 URL
        plugin: String,
    },

    /// 移除插件
    Remove {
        /// 插件名称
        name: String,
    },

    /// 更新所有插件
    Update,

    /// 搜索插件
    Search {
        /// 搜索查询
        query: String,
    },

    /// 启用插件
    Enable {
        /// 插件名称
        name: String,
    },

    /// 禁用插件
    Disable {
        /// 插件名称
        name: String,
    },
}

/// 记忆子命令
#[derive(Subcommand, Debug)]
pub enum MemoryCommands {
    /// 显示记忆状态
    Status,

    /// 清空所有记忆
    Clear,

    /// 导出记忆
    Export {
        /// 输出文件路径
        #[arg(short, long)]
        output: PathBuf,
    },

    /// 导入记忆
    Import {
        /// 输入文件路径
        input: PathBuf,
    },

    /// 运行记忆整合（dream）
    Dream,

    /// 强制 AutoDream 整合
    AutoDream,
}

/// 服务子命令
#[derive(Subcommand, Debug)]
pub enum ServiceCommands {
    /// 显示所有服务状态
    Status,

    /// 启动所有服务
    Start,

    /// 停止所有服务
    Stop,

    /// 检查 AutoDream 状态
    AutoDream,

    /// 检查语音服务状态
    Voice,

    /// 检查 Magic Docs 状态
    MagicDocs,

    /// 检查团队同步状态
    TeamSync,

    /// 检查插件市场状态
    Plugins,

    /// 检查 Agent 服务状态
    Agents,
}

/// Magic Docs 子命令
#[derive(Subcommand, Debug)]
pub enum MagicDocsCommands {
    /// 列出追踪的 Magic Docs
    List,

    /// 检查文件的 Magic Doc 头部
    Check {
        /// 要检查的文件路径
        file: String,
    },

    /// 更新 Magic Doc
    Update {
        /// 要更新的文件路径
        file: String,
        /// 更新上下文
        #[arg(short, long)]
        context: Option<String>,
    },

    /// 清空所有追踪的 Magic Docs
    Clear,
}

/// 团队同步子命令
#[derive(Subcommand, Debug)]
pub enum TeamSyncCommands {
    /// 显示同步状态
    Status,

    /// 团队认证
    Auth {
        /// 团队 ID
        team_id: String,
    },

    /// 同步记忆
    Sync,

    /// 列出团队记忆
    List,

    /// 创建团队记忆
    Create {
        /// 记忆标题
        title: String,
        /// 记忆内容
        #[arg(short, long)]
        content: String,
        /// 标签（逗号分隔）
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// 删除团队记忆
    Delete {
        /// 记忆 ID
        id: String,
    },
}

/// Skills 子命令
#[derive(Subcommand, Debug)]
pub enum SkillsCommands {
    /// 列出所有可用技能
    List,

    /// 执行技能
    Execute {
        /// 技能名称
        skill: String,
        /// 技能参数
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// 获取技能帮助
    Help {
        /// 技能名称
        skill: String,
    },

    /// 搜索技能
    Search {
        /// 搜索查询
        query: String,
    },
}
