//! 高级功能模块
//!
//! 提供高级功能支持，包括：
//! - SSH 连接支持
//! - 远程执行
//! - 项目初始化

pub mod ssh;           // SSH 连接支持
pub mod remote;        // 远程执行
pub mod project_init;  // 项目初始化

use serde::{Deserialize, Serialize};

pub use ssh::{SshClient, SshConfig, SshSession};
pub use remote::{RemoteExecutor, RemoteConfig, RemoteResult};
pub use project_init::{ProjectInitializer, ProjectConfig, ProjectTemplate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    pub ssh: SshConfig,
    pub remote: RemoteConfig,
    pub project: ProjectConfig,
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            ssh: SshConfig::default(),
            remote: RemoteConfig::default(),
            project: ProjectConfig::default(),
        }
    }
}
