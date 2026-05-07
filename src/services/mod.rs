//! 服务模块 - Claude Code 后台服务
//!
//! 本模块提供各种后台服务，包括：
//! - AutoDream: 自动记忆整合
//! - Voice: 语音输入与转录
//! - MagicDocs: 自动文档维护
//! - TeamMemorySync: 团队记忆同步
//! - PluginMarketplace: 插件市场
//! - Agents: 内置 Agent 系统

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::state::AppState;

pub mod auto_dream;        // 自动记忆整合服务
pub mod voice;            // 语音服务
pub mod magic_docs;       // Magic Docs 服务
pub mod team_memory_sync; // 团队记忆同步
pub mod plugin_marketplace; // 插件市场
pub mod agents;           // Agent 服务
pub mod stress_tests;    // 压力测试

pub use auto_dream::{AutoDreamService, AutoDreamConfig, AutoDreamStatus};
pub use voice::{VoiceService, VoiceConfig, VoiceBackend, VoiceStatus, RecordingState};
pub use magic_docs::{MagicDocsService, MagicDocsConfig, MagicDocInfo, MagicDocHeader};
pub use team_memory_sync::{TeamMemorySyncService, TeamMemoryConfig, TeamMemorySyncStatus, TeamMemory, ConflictResolution};
pub use plugin_marketplace::{PluginMarketplaceService, PluginConfig, Plugin, MarketplacePlugin};
pub use agents::{AgentsService, AgentDefinition, AgentType, AgentSession, AgentStatus};
pub use stress_tests::{StressTestRunner, StressTestResult, run_stress_test};

/// 后台服务管理器
pub struct ServiceManager {
    state: Arc<RwLock<AppState>>,
    auto_dream: Option<Arc<AutoDreamService>>,
    voice: Option<Arc<VoiceService>>,
    magic_docs: Option<Arc<MagicDocsService>>,
    team_memory_sync: Option<Arc<TeamMemorySyncService>>,
    plugin_marketplace: Option<Arc<PluginMarketplaceService>>,
    agents: Option<Arc<AgentsService>>,
}

impl ServiceManager {
    /// 创建新的服务管理器
    pub fn new(state: Arc<RwLock<AppState>>) -> Self {
        Self {
            state,
            auto_dream: None,
            voice: None,
            magic_docs: None,
            team_memory_sync: None,
            plugin_marketplace: None,
            agents: None,
        }
    }

    /// 初始化所有服务
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        println!("🔧 正在初始化服务...");

        self.auto_dream = Some(Arc::new(AutoDreamService::new(self.state.clone(), None)));
        self.voice = Some(Arc::new(VoiceService::new(self.state.clone(), None)));
        self.magic_docs = Some(Arc::new(MagicDocsService::new(self.state.clone(), None)));
        self.team_memory_sync = Some(Arc::new(TeamMemorySyncService::new(self.state.clone(), None)));
        self.plugin_marketplace = Some(Arc::new(PluginMarketplaceService::new(self.state.clone(), None)));
        self.agents = Some(Arc::new(AgentsService::new(self.state.clone())));

        if let Some(magic_docs) = &self.magic_docs {
            magic_docs.load_state().await?;
        }

        println!("✅ 服务初始化完成");
        Ok(())
    }

    /// 启动所有后台服务
    pub async fn start_all(&self) -> anyhow::Result<()> {
        println!("🚀 正在启动后台服务...");

        if let Some(auto_dream) = &self.auto_dream {
            let status = auto_dream.get_status().await;
            println!("   🌙 AutoDream: {} (上次: {}小时前)",
                     if status.enabled { "已启用" } else { "已禁用" },
                     status.hours_since_last);
        }

        if let Some(voice) = &self.voice {
            let status = voice.get_status().await;
            println!("   🎤 语音: {} ({:?})",
                     if status.available { "可用" } else { "不可用" },
                     status.backend);
        }

        if let Some(magic_docs) = &self.magic_docs {
            let status = magic_docs.get_status().await;
            println!("   📚 MagicDocs: 追踪 {} 个文档", status.tracked_count);
        }

        if let Some(team_sync) = &self.team_memory_sync {
            let status = team_sync.get_status().await;
            println!("   👥 团队同步: {} 本地, {} 远程",
                     status.local_memories, status.remote_memories);
        }

        if let Some(plugins) = &self.plugin_marketplace {
            let status = plugins.get_status().await;
            println!("   🔌 插件: 已安装 {} 个", status.installed_count);
        }

        if let Some(agents) = &self.agents {
            let status = agents.get_status().await;
            println!("   🤖 Agent: {} 可用, {} 活跃",
                     status.available_agents.len(), status.active_sessions);
        }

        println!("✅ 所有服务已启动");
        Ok(())
    }

    /// 停止所有后台服务
    pub async fn stop_all(&self) -> anyhow::Result<()> {
        println!("🛑 正在停止后台服务...");

        if let Some(magic_docs) = &self.magic_docs {
            magic_docs.save_state().await?;
        }

        println!("✅ 所有服务已停止");
        Ok(())
    }

    /// 获取 AutoDream 服务
    pub fn auto_dream(&self) -> Option<Arc<AutoDreamService>> {
        self.auto_dream.clone()
    }

    /// 获取语音服务
    pub fn voice(&self) -> Option<Arc<VoiceService>> {
        self.voice.clone()
    }

    /// 获取 Magic Docs 服务
    pub fn magic_docs(&self) -> Option<Arc<MagicDocsService>> {
        self.magic_docs.clone()
    }

    /// 获取团队同步服务
    pub fn team_memory_sync(&self) -> Option<Arc<TeamMemorySyncService>> {
        self.team_memory_sync.clone()
    }

    /// 获取插件市场服务
    pub fn plugin_marketplace(&self) -> Option<Arc<PluginMarketplaceService>> {
        self.plugin_marketplace.clone()
    }

    /// 获取 Agent 服务
    pub fn agents(&self) -> Option<Arc<AgentsService>> {
        self.agents.clone()
    }

    /// 获取所有服务状态
    pub async fn get_status(&self) -> ServiceStatus {
        ServiceStatus {
            auto_dream: self.auto_dream.as_ref().map(|s| futures::executor::block_on(s.get_status())),
            voice: self.voice.as_ref().map(|s| futures::executor::block_on(s.get_status())),
            magic_docs: self.magic_docs.as_ref().map(|s| futures::executor::block_on(s.get_status())),
            team_sync: self.team_memory_sync.as_ref().map(|s| futures::executor::block_on(s.get_status())),
            plugins: self.plugin_marketplace.as_ref().map(|s| futures::executor::block_on(s.get_status())),
            agents: self.agents.as_ref().map(|s| futures::executor::block_on(s.get_status())),
        }
    }
}

/// 所有服务状态汇总
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceStatus {
    pub auto_dream: Option<AutoDreamStatus>,
    pub voice: Option<VoiceStatus>,
    pub magic_docs: Option<magic_docs::MagicDocsStatus>,
    pub team_sync: Option<TeamMemorySyncStatus>,
    pub plugins: Option<plugin_marketplace::PluginStatus>,
    pub agents: Option<agents::AgentStatusReport>,
}
