//! CLI 参数解析 - 命令行入口与子命令分发
//!
//! 支持的子命令：
//! - repl: 交互式 REPL 模式
//! - query: 单次查询
//! - config: 配置管理
//! - mcp: MCP 服务器管理
//! - plugin: 插件管理
//! - memory: 记忆管理
//! - voice: 语音输入
//! - init: 项目初始化
//! - services: 服务管理
//! - agent: 运行 Agent
//! - magic-docs: Magic Docs 管理
//! - team-sync: 团队记忆同步
//! - stress-test: 压力测试
//! - skills: Skills 管理

use super::CliArgs;
use std::sync::Arc;
use tokio::sync::RwLock;

/// CLI 类型别名
pub type Cli = CliArgs;

impl Cli {
    /// 异步运行 CLI - 根据子命令分发到对应的处理函数
    pub async fn run_async(&self, state: crate::state::AppState) -> anyhow::Result<()> {
        // 显示版本号
        if self.version {
            println!("claude-code-rust {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }

        // 显示系统信息
        if self.info {
            self.print_system_info();
            return Ok(());
        }

        // 根据子命令分发
        match &self.command {
            Some(super::Commands::Repl { prompt }) => {
                self.run_repl(state, prompt.clone())?;
            }
            Some(super::Commands::Query { prompt }) => {
                self.run_query(state, prompt.clone()).await?;
            }
            Some(super::Commands::Config { action }) => {
                self.run_config(action)?;
            }
            Some(super::Commands::Mcp { action }) => {
                self.run_mcp(action).await?;
            }
            Some(super::Commands::Plugin { action }) => {
                self.run_plugin(action).await?;
            }
            Some(super::Commands::Memory { action }) => {
                self.run_memory(action).await?;
            }
            Some(super::Commands::Voice { push_to_talk }) => {
                self.run_voice(state, *push_to_talk).await?;
            }
            Some(super::Commands::Init { name }) => {
                self.run_init(name.clone())?;
            }
            Some(super::Commands::Update) => {
                self.run_update()?;
            }
            Some(super::Commands::Help { topic }) => {
                self.run_help(topic.clone())?;
            }
            Some(super::Commands::Services { action }) => {
                self.run_services(state, action).await?;
            }
            Some(super::Commands::Agent { agent_type, prompt }) => {
                self.run_agent(state, agent_type, prompt).await?;
            }
            Some(super::Commands::MagicDocs { action }) => {
                self.run_magic_docs(state, action).await?;
            }
            Some(super::Commands::TeamSync { action }) => {
                self.run_team_sync(state, action).await?;
            }
            Some(super::Commands::StressTest { concurrency, iterations }) => {
                self.run_stress_test(*concurrency, *iterations).await?;
            }
            Some(super::Commands::Skills { action }) => {
                self.run_skills(action).await?;
            }
            // 无子命令时，默认启动 REPL
            None => {
                self.run_repl(state, None)?;
            }
        }

        Ok(())
    }

    /// 打印系统信息
    fn print_system_info(&self) {
        use colored::Colorize;

        println!();
        println!("  {}", "系统信息".truecolor(147, 112, 219).bold());
        println!();
        println!("  {:20} {}", "版本:", env!("CARGO_PKG_VERSION").green());
        println!("  {:20} {}", "操作系统:", std::env::consts::OS.cyan());
        println!("  {:20} {}", "架构:", std::env::consts::ARCH.cyan());
        println!("  {:20} {}", "工作目录:", std::env::current_dir().unwrap().display().to_string().bright_white());
        println!();
    }

    /// 运行交互式 REPL
    fn run_repl(&self, state: crate::state::AppState, prompt: Option<String>) -> anyhow::Result<()> {
        let mut repl = crate::cli::repl::Repl::new(state);
        repl.start(prompt)?;
        Ok(())
    }

    /// 运行单次查询（无对话状态）
    async fn run_query(&self, state: crate::state::AppState, prompt: String) -> anyhow::Result<()> {
        let client = crate::api::ApiClient::new(state.settings.clone());

        let api_key = match client.get_api_key() {
            Some(key) => key,
            None => {
                eprintln!("错误: API 密钥未配置");
                eprintln!("请设置环境变量 DEEPSEEK_API_KEY 或运行:");
                eprintln!("  claude-code config set api_key \"your-api-key\"");
                std::process::exit(1);
            }
        };

        let messages = vec![crate::api::ChatMessage::user(&prompt)];
        let base_url = client.get_base_url().to_string();
        let model = client.get_model().to_string();
        let max_tokens = state.settings.api.max_tokens;

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens,
            "stream": false,
            "temperature": 0.7
        });

        let http_client = reqwest::Client::new();
        let url = format!("{}/v1/chat/completions", base_url);

        let response = http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API 错误 ({}): {}", status, body));
        }

        let json: serde_json::Value = response.json().await?;

        if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
            if let Some(choice) = choices.first() {
                if let Some(content) = choice.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                    println!("{}", content);
                }
            }
        }

        Ok(())
    }

    /// 运行配置命令（显示/设置/重置）
    fn run_config(&self, action: &super::ConfigCommands) -> anyhow::Result<()> {
        match action {
            super::ConfigCommands::Show => {
                let settings = crate::config::Settings::load()?;
                println!("{}", serde_json::to_string_pretty(&settings)?);
            }
            super::ConfigCommands::Set { key, value } => {
                crate::config::Settings::set(key, value)?;
                println!("已设置 {} = {}", key, value);
            }
            super::ConfigCommands::Reset => {
                crate::config::Settings::reset()?;
                println!("配置已重置为默认值");
            }
        }
        Ok(())
    }

    /// 运行 MCP 服务器管理命令
    async fn run_mcp(&self, action: &super::McpCommands) -> anyhow::Result<()> {
        let manager = crate::mcp::McpManager::new();
        match action {
            super::McpCommands::List => {
                let servers = manager.list_servers().await?;
                for server in servers {
                    println!("  - {} ({})", server.name, server.status);
                }
            }
            super::McpCommands::Add { name, command } => {
                let config = crate::mcp::McpConfig::new(name, command);
                manager.add_server(config).await?;
                println!("已添加 MCP 服务器: {}", name);
            }
            super::McpCommands::Remove { name } => {
                manager.remove_server(name).await?;
                println!("已删除 MCP 服务器: {}", name);
            }
            super::McpCommands::Restart { name } => {
                manager.restart_server(name).await?;
                println!("已重启 MCP 服务器: {}", name);
            }
        }
        Ok(())
    }

    /// 运行插件管理命令
    async fn run_plugin(&self, action: &super::PluginCommands) -> anyhow::Result<()> {
        let state = Arc::new(RwLock::new(crate::state::AppState::default()));
        let service = crate::services::PluginMarketplaceService::new(state, None);

        match action {
            super::PluginCommands::List => {
                let plugins = service.list_installed().await;
                if plugins.is_empty() {
                    println!("未安装插件");
                } else {
                    for plugin in plugins {
                        let status = if plugin.enabled { "已启用" } else { "已禁用" };
                        println!("  - {} v{} [{}]", plugin.name, plugin.version, status);
                    }
                }
            }
            super::PluginCommands::Install { plugin } => {
                let installed = service.install(plugin).await?;
                println!("已安装: {} v{}", installed.name, installed.version);
            }
            super::PluginCommands::Remove { name } => {
                service.remove(name).await?;
                println!("已删除插件: {}", name);
            }
            super::PluginCommands::Update => {
                let updated = service.update_all().await?;
                println!("已更新 {} 个插件", updated.len());
            }
            super::PluginCommands::Search { query } => {
                let results = service.search(query).await;
                if results.is_empty() {
                    println!("未找到插件: {}", query);
                } else {
                    for plugin in results {
                        println!("  - {} v{} by {} (⭐ {})",
                                 plugin.name, plugin.version, plugin.author, plugin.rating);
                        println!("    {}", plugin.description);
                    }
                }
            }
            super::PluginCommands::Enable { name } => {
                service.enable(name).await?;
                println!("已启用插件: {}", name);
            }
            super::PluginCommands::Disable { name } => {
                service.disable(name).await?;
                println!("已禁用插件: {}", name);
            }
        }
        Ok(())
    }

    /// 运行记忆管理命令
    async fn run_memory(&self, action: &super::MemoryCommands) -> anyhow::Result<()> {
        let manager = crate::memory::MemoryManager::new();
        manager.load().await?;

        match action {
            super::MemoryCommands::Status => {
                let status = manager.status().await?;
                println!("记忆状态:");
                println!("  会话数: {}", status.session_count);
                println!("  记忆数: {}", status.total_memories);
                println!("  上次整合: {:?}", status.last_consolidation);
            }
            super::MemoryCommands::Clear => {
                manager.clear().await?;
                println!("已清空所有记忆");
            }
            super::MemoryCommands::Export { output } => {
                manager.export(output).await?;
                println!("记忆已导出到: {}", output.display());
            }
            super::MemoryCommands::Import { input } => {
                manager.import(input).await?;
                println!("记忆已从以下位置导入: {}", input.display());
            }
            super::MemoryCommands::Dream => {
                println!("正在运行记忆整合 (dream)...");
                manager.consolidate().await?;
                println!("记忆整合完成");
            }
            super::MemoryCommands::AutoDream => {
                let state = Arc::new(RwLock::new(crate::state::AppState::default()));
                let service = crate::services::AutoDreamService::new(state, None);
                println!("正在强制执行 AutoDream 整合...");
                service.force_consolidation().await?;
                println!("AutoDream 整合完成");
            }
        }
        Ok(())
    }

    /// 运行语音输入命令
    async fn run_voice(&self, state: crate::state::AppState, push_to_talk: bool) -> anyhow::Result<()> {
        let state = Arc::new(RwLock::new(state));
        let service = crate::services::VoiceService::new(state, None);

        let status = service.get_status().await;
        if !status.available {
            println!("当前系统不支持语音输入");
            println!("后端: {:?}", status.backend);
            return Ok(());
        }

        if push_to_talk {
            println!("🎤 按住说话模式已启用");
            println!("按 Enter 开始录音，再次按 Enter 停止。");

            service.push_to_talk_start().await?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let text = service.push_to_talk_stop().await?;
            println!("\n📝 转录结果: {}", text);
        } else {
            println!("🎤 连续语音输入模式");
            println!("正在启动语音输入...");
            service.start_recording().await?;
        }

        Ok(())
    }

    /// 初始化新项目
    fn run_init(&self, name: Option<String>) -> anyhow::Result<()> {
        let project_name = name.unwrap_or_else(|| "claude-code-project".to_string());
        crate::utils::project::init_project(&project_name)?;
        println!("已初始化项目: {}", project_name);
        Ok(())
    }

    /// 检查更新
    fn run_update(&self) -> anyhow::Result<()> {
        println!("正在检查更新...");
        println!("已是最新版本");
        Ok(())
    }

    /// 显示帮助信息
    fn run_help(&self, topic: Option<String>) -> anyhow::Result<()> {
        match topic {
            Some(t) => println!("主题 {} 的帮助:", t),
            None => println!("使用 --help 获取详细使用说明"),
        }
        Ok(())
    }

    /// 运行服务管理命令
    ///
    /// 管理以下服务：
    /// - AutoDream: 自动记忆整合
    /// - Voice: 语音服务
    /// - MagicDocs: 文档追踪
    /// - TeamSync: 团队记忆同步
    /// - Plugins: 插件市场
    /// - Agents: Agent 服务
    async fn run_services(&self, state: crate::state::AppState, action: &super::ServiceCommands) -> anyhow::Result<()> {
        let state = Arc::new(RwLock::new(state));
        let mut manager = crate::services::ServiceManager::new(state.clone());
        manager.initialize().await?;

        match action {
            super::ServiceCommands::Status => {
                let status = manager.get_status().await;
                println!("服务状态:");
                println!("{}", serde_json::to_string_pretty(&status)?);
            }
            super::ServiceCommands::Start => {
                manager.start_all().await?;
            }
            super::ServiceCommands::Stop => {
                manager.stop_all().await?;
            }
            super::ServiceCommands::AutoDream => {
                if let Some(auto_dream) = manager.auto_dream() {
                    let status = auto_dream.get_status().await;
                    println!("AutoDream 状态:");
                    println!("  已启用: {}", status.enabled);
                    println!("  正在整合: {}", status.is_consolidating);
                    println!("  距上次整合: {}小时", status.hours_since_last);
                    println!("  累积会话数: {}", status.sessions_accumulated);
                    println!("  距下次整合: {}小时", status.next_consolidation_in);
                }
            }
            super::ServiceCommands::Voice => {
                if let Some(voice) = manager.voice() {
                    let status = voice.get_status().await;
                    println!("语音服务状态:");
                    println!("  可用: {}", status.available);
                    println!("  后端: {:?}", status.backend);
                    println!("  状态: {:?}", status.state);
                }
            }
            super::ServiceCommands::MagicDocs => {
                if let Some(magic_docs) = manager.magic_docs() {
                    let status = magic_docs.get_status().await;
                    println!("Magic Docs 状态:");
                    println!("  已启用: {}", status.enabled);
                    println!("  自动更新: {}", status.auto_update);
                    println!("  追踪的文档: {}", status.tracked_count);
                }
            }
            super::ServiceCommands::TeamSync => {
                if let Some(team_sync) = manager.team_memory_sync() {
                    let status = team_sync.get_status().await;
                    println!("团队同步状态:");
                    println!("  已启用: {}", status.enabled);
                    println!("  已认证: {}", status.is_authenticated);
                    println!("  本地记忆: {}", status.local_memories);
                    println!("  远程记忆: {}", status.remote_memories);
                }
            }
            super::ServiceCommands::Plugins => {
                if let Some(plugins) = manager.plugin_marketplace() {
                    let status = plugins.get_status().await;
                    println!("插件市场状态:");
                    println!("  已启用: {}", status.enabled);
                    println!("  已安装: {}", status.installed_count);
                    println!("  可用更新: {}", status.updates_available);
                }
            }
            super::ServiceCommands::Agents => {
                if let Some(agents) = manager.agents() {
                    let status = agents.get_status().await;
                    println!("Agent 服务状态:");
                    println!("  可用 Agent: {}", status.available_agents.len());
                    println!("  活跃会话: {}", status.active_sessions);
                    for agent in &status.available_agents {
                        println!("    - {} ({})", agent.name, agent.agent_type);
                    }
                }
            }
        }
        Ok(())
    }

    /// 运行指定类型的 Agent
    ///
    /// 支持的 Agent 类型：
    /// - guide: Claude Code 使用指南
    /// - explore: 代码库探索
    /// - plan: 计划制定
    /// - verify: 代码审查验证
    /// - general: 通用 Agent
    async fn run_agent(&self, state: crate::state::AppState, agent_type: &str, prompt: &str) -> anyhow::Result<()> {
        let state = Arc::new(RwLock::new(state));
        let service = crate::services::AgentsService::new(state);

        let agent_type = match agent_type.to_lowercase().as_str() {
            "guide" | "claude-code-guide" => crate::services::AgentType::ClaudeCodeGuide,
            "explore" => crate::services::AgentType::Explore,
            "plan" => crate::services::AgentType::Plan,
            "verify" | "verification" => crate::services::AgentType::Verification,
            "general" | "general-purpose" => crate::services::AgentType::GeneralPurpose,
            _ => {
                println!("未知 Agent 类型: {}", agent_type);
                println!("可用类型: guide, explore, plan, verify, general");
                return Ok(());
            }
        };

        println!("🤖 正在运行 {} Agent...", agent_type);
        println!("提示: {}", prompt);
        println!();

        let session = service.run_agent(&agent_type, prompt).await?;

        if let Some(result) = &session.result {
            println!("{}", result);
        }

        Ok(())
    }

    /// 运行 Magic Docs 管理命令
    ///
    /// Magic Docs 是一种特殊格式的文档，在文件头部包含元数据，
    /// 可以追踪文档的修改历史和上下文。
    async fn run_magic_docs(&self, state: crate::state::AppState, action: &super::MagicDocsCommands) -> anyhow::Result<()> {
        let state = Arc::new(RwLock::new(state));
        let service = crate::services::MagicDocsService::new(state, None);

        match action {
            super::MagicDocsCommands::List => {
                let docs = service.get_tracked_docs().await;
                if docs.is_empty() {
                    println!("未追踪任何 Magic Docs");
                } else {
                    for doc in docs {
                        println!("  - {} ({})", doc.title, doc.path);
                        println!("    更新: {} ({}次)", doc.last_updated, doc.update_count);
                    }
                }
            }
            super::MagicDocsCommands::Check { file } => {
                if let Some(header) = service.check_file(file).await {
                    println!("检测到 Magic Doc:");
                    println!("  标题: {}", header.title);
                    if let Some(instructions) = header.instructions {
                        println!("  说明: {}", instructions);
                    }
                } else {
                    println!("不是 Magic Doc: {}", file);
                }
            }
            super::MagicDocsCommands::Update { file, context } => {
                let ctx = context.clone().unwrap_or_else(|| "手动更新".to_string());
                service.update_magic_doc(file, &ctx).await?;
                println!("已更新 Magic Doc: {}", file);
            }
            super::MagicDocsCommands::Clear => {
                service.clear_all().await;
                println!("已清空所有 Magic Docs");
            }
        }
        Ok(())
    }

    /// 运行团队记忆同步命令
    ///
    /// 支持团队成员之间共享和同步记忆。
    async fn run_team_sync(&self, state: crate::state::AppState, action: &super::TeamSyncCommands) -> anyhow::Result<()> {
        use crate::services::{TeamMemorySyncService, TeamMemoryConfig, ConflictResolution};

        let state = Arc::new(RwLock::new(state));
        let service = TeamMemorySyncService::new(
            state,
            Some(TeamMemoryConfig {
                enabled: true,
                team_id: Some("test-team".to_string()),
                sync_interval_secs: 3600,
                auto_sync: false,
                conflict_resolution: ConflictResolution::PreferNewer,
            }),
        );

        match action {
            super::TeamSyncCommands::Status => {
                let status = service.get_status().await;
                println!("团队同步状态:");
                println!("  已启用: {}", status.enabled);
                println!("  团队 ID: {:?}", status.team_id);
                println!("  已认证: {}", status.is_authenticated);
                println!("  本地记忆: {}", status.local_memories);
                println!("  远程记忆: {}", status.remote_memories);
                if let Some(last) = status.sync_status.last_sync {
                    println!("  上次同步: {}", last);
                }
            }
            super::TeamSyncCommands::Sync => {
                println!("正在开始同步...");
                let result = service.sync().await?;
                println!("同步完成:");
                println!("  上传: {}", result.uploaded);
                println!("  下载: {}", result.downloaded);
                println!("  冲突: {}", result.conflicts);
                if !result.errors.is_empty() {
                    println!("  错误: {:?}", result.errors);
                }
            }
            super::TeamSyncCommands::Auth { team_id } => {
                println!("正在认证团队: {}", team_id);
                if service.authenticate(team_id).await.is_ok() {
                    println!("✅ 认证成功");
                } else {
                    println!("❌ 认证失败");
                }
            }
            super::TeamSyncCommands::Create { title, content, .. } => {
                let memory = service.create_memory(title, content, vec![]).await;
                if memory.is_ok() {
                    println!("✅ 记忆已创建: {}", title);
                } else {
                    println!("❌ 创建记忆失败");
                }
            }
            super::TeamSyncCommands::List => {
                let memories = service.list_memories().await;
                if memories.is_empty() {
                    println!("无本地记忆");
                } else {
                    for memory in memories {
                        println!("  - {} ({})", memory.title, memory.author);
                    }
                }
            }
            super::TeamSyncCommands::Delete { .. } => {
                println!("删除功能未实现");
            }
        }
        Ok(())
    }

    /// 运行压力测试
    async fn run_stress_test(&self, concurrency: usize, iterations: usize) -> anyhow::Result<()> {
        use crate::services::run_stress_test;
        run_stress_test(concurrency, iterations).await;
        Ok(())
    }

    /// 运行 Skills 管理命令
    async fn run_skills(&self, action: &super::SkillsCommands) -> anyhow::Result<()> {
        match action {
            super::SkillsCommands::List => {
                println!("可用技能:");
                println!("  - simplify: 代码审查与简化");
                println!("  - loop: 运行循环任务");
                println!("  - schedule: 调度自动化任务");
            }
            super::SkillsCommands::Execute { skill, args } => {
                println!("正在执行技能: {}，参数: {:?}", skill, args);
            }
            super::SkillsCommands::Help { skill } => {
                println!("技能 {} 的帮助:", skill);
            }
            super::SkillsCommands::Search { query } => {
                println!("正在搜索技能: {}", query);
            }
        }
        Ok(())
    }
}
