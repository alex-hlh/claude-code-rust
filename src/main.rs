//! Claude Code Rust - 主程序入口
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

use clap::Parser;
use claude_code_rs::cli::Cli;
use claude_code_rs::config::Settings;
use claude_code_rs::state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 主函数 - 应用程序入口点
/// 使用 #[tokio::main] 宏将 async 函数转换为同步入口点
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志系统，支持从环境变量读取日志级别
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 解析命令行参数
    let cli = Cli::parse();

    // 从配置文件加载设置（默认路径：~/.claude-code/settings.json）
    let settings = Settings::load()?;

    // 创建应用状态，包含会话历史、当前对话、工具注册表等共享状态
    let state = AppState::new(settings);

    // 根据子命令分发到对应的处理函数
    match cli.run_async(state).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
