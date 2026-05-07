//! 插件模块 - 完整的插件系统
//!
//! 插件功能包括：
//! - 自定义命令
//! - 钩子系统
//! - 热加载支持
//! - 插件隔离

pub mod commands;   // 插件命令
pub mod hooks;     // 钩子系统
pub mod loader;    // 插件加载器
pub mod isolation;  // 插件隔离（沙箱）
pub mod registry;  // 插件注册表

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, Utc};

pub use commands::{PluginCommand, CommandRegistry};
pub use hooks::{Hook, HookManager, HookPoint};
pub use loader::{PluginLoader, LoadedPlugin};
pub use isolation::{PluginSandbox, IsolationConfig};
pub use registry::PluginRegistry;

/// 插件清单 - 描述插件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub main: String,
    pub commands: Vec<PluginCommandDef>,
    pub hooks: Vec<String>,
    pub dependencies: HashMap<String, String>,
    pub permissions: Vec<String>,
    pub enabled: bool,
}

impl PluginManifest {
    /// 创建新的插件清单
    pub fn new(name: &str, version: &str, main: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: None,
            author: None,
            license: None,
            repository: None,
            main: main.to_string(),
            commands: Vec::new(),
            hooks: Vec::new(),
            dependencies: HashMap::new(),
            permissions: Vec::new(),
            enabled: true,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    /// 添加命令
    pub fn with_command(mut self, command: PluginCommandDef) -> Self {
        self.commands.push(command);
        self
    }

    /// 添加钩子
    pub fn with_hook(mut self, hook: &str) -> Self {
        self.hooks.push(hook.to_string());
        self
    }

    /// 添加权限
    pub fn with_permission(mut self, permission: &str) -> Self {
        self.permissions.push(permission.to_string());
        self
    }
}

/// 插件命令定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommandDef {
    pub name: String,
    pub description: String,
    pub usage: Option<String>,
    pub examples: Vec<String>,
}

impl PluginCommandDef {
    /// 创建新的命令定义
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            usage: None,
            examples: Vec::new(),
        }
    }
}

/// 插件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub status: PluginStatus,
    pub enabled: bool,
    pub loaded_at: Option<DateTime<Utc>>,
    pub commands_count: usize,
    pub hooks_count: usize,
}

/// 插件状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginStatus {
    Installed,  // 已安装
    Loaded,     // 已加载
    Error,      // 错误
    Disabled,   // 已禁用
}

/// 插件管理器 - 管理插件的安装、加载、卸载
pub struct PluginManager {
    registry: Arc<PluginRegistry>,
    loader: Arc<PluginLoader>,
    sandbox: Arc<PluginSandbox>,
    hook_manager: Arc<HookManager>,
    command_registry: Arc<CommandRegistry>,
    plugins_dir: PathBuf,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let plugins_dir = home.join(".claude-code").join("plugins");

        Self {
            registry: Arc::new(PluginRegistry::new()),
            loader: Arc::new(PluginLoader::new()),
            sandbox: Arc::new(PluginSandbox::new(Default::default())),
            hook_manager: Arc::new(HookManager::new()),
            command_registry: Arc::new(CommandRegistry::new()),
            plugins_dir,
        }
    }

    /// 设置插件目录
    pub fn with_plugins_dir(mut self, dir: PathBuf) -> Self {
        self.plugins_dir = dir;
        self
    }

    /// 列出所有插件
    pub async fn list(&self) -> anyhow::Result<Vec<PluginInfo>> {
        let plugins = self.registry.list().await;
        Ok(plugins)
    }

    /// 安装插件
    pub async fn install(&self, source: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.plugins_dir)?;

        let plugin_name = source.rsplit('/').next().unwrap_or(source);
        let plugin_dir = self.plugins_dir.join(plugin_name);

        if source.starts_with("http") || source.starts_with("git") {
            println!("📥 从 {} 克隆插件...", source);
            let output = tokio::process::Command::new("git")
                .args(["clone", source, &plugin_dir.to_string_lossy()])
                .output()
                .await?;

            if !output.status.success() {
                return Err(anyhow::anyhow!("克隆插件失败: {}",
                    String::from_utf8_lossy(&output.stderr)));
            }
        } else if std::path::Path::new(source).exists() {
            println!("📁 从 {} 复制插件...", source);
            fs_extra::dir::copy(source, &self.plugins_dir, &fs_extra::dir::CopyOptions::new())?;
        } else {
            return Err(anyhow::anyhow!("未找到插件源: {}", source));
        }

        let manifest = self.loader.load_manifest(&plugin_dir).await?;
        self.registry.register(manifest).await?;

        println!("✅ 插件已安装: {}", plugin_name);
        Ok(())
    }

    /// 移除插件
    pub async fn remove(&self, name: &str) -> anyhow::Result<()> {
        self.registry.unregister(name).await?;

        let plugin_dir = self.plugins_dir.join(name);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(&plugin_dir)?;
        }

        println!("🗑️ 插件已删除: {}", name);
        Ok(())
    }

    /// 加载插件
    pub async fn load(&self, name: &str) -> anyhow::Result<()> {
        let manifest = self.registry.get(name).await?
            .ok_or_else(|| anyhow::anyhow!("未找到插件: {}", name))?;

        let plugin_dir = self.plugins_dir.join(name);
        let loaded = self.loader.load(&plugin_dir, &manifest).await?;

        // 注册命令
        for cmd in &manifest.commands {
            self.command_registry.register(cmd.clone()).await;
        }

        // 注册钩子
        for hook in &manifest.hooks {
            self.hook_manager.register(hook.parse()?, name).await;
        }

        self.registry.set_loaded(name, loaded).await?;
        println!("✅ 插件已加载: {}", name);
        Ok(())
    }

    /// 卸载插件
    pub async fn unload(&self, name: &str) -> anyhow::Result<()> {
        let manifest = self.registry.get(name).await?
            .ok_or_else(|| anyhow::anyhow!("未找到插件: {}", name))?;

        for cmd in &manifest.commands {
            self.command_registry.unregister(&cmd.name).await;
        }

        for hook in &manifest.hooks {
            self.hook_manager.unregister(&hook.parse()?, name).await;
        }

        self.registry.set_unloaded(name).await?;
        println!("⏹️ 插件已卸载: {}", name);
        Ok(())
    }

    /// 重新加载插件
    pub async fn reload(&self, name: &str) -> anyhow::Result<()> {
        self.unload(name).await?;
        self.load(name).await
    }

    /// 启用插件
    pub async fn enable(&self, name: &str) -> anyhow::Result<()> {
        self.registry.set_enabled(name, true).await?;
        self.load(name).await
    }

    /// 禁用插件
    pub async fn disable(&self, name: &str) -> anyhow::Result<()> {
        self.unload(name).await?;
        self.registry.set_enabled(name, false).await
    }

    /// 更新插件
    pub async fn update(&self, name: &str) -> anyhow::Result<()> {
        let plugin_dir = self.plugins_dir.join(name);

        if plugin_dir.join(".git").exists() {
            println!("⬆️ 正在更新插件: {}", name);
            let output = tokio::process::Command::new("git")
                .args(["pull"])
                .current_dir(&plugin_dir)
                .output()
                .await?;

            if output.status.success() {
                self.reload(name).await?;
                println!("✅ 插件已更新: {}", name);
            } else {
                return Err(anyhow::anyhow!("更新插件失败: {}",
                    String::from_utf8_lossy(&output.stderr)));
            }
        } else {
            println!("⚠️ 插件不是 git 仓库，跳过更新");
        }

        Ok(())
    }

    /// 更新所有插件
    pub async fn update_all(&self) -> anyhow::Result<()> {
        let plugins = self.registry.list().await;
        for plugin in plugins {
            if plugin.enabled {
                let _ = self.update(&plugin.name).await;
            }
        }
        Ok(())
    }

    /// 加载所有插件
    pub async fn load_all(&self) -> anyhow::Result<()> {
        if !self.plugins_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&self.plugins_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Err(e) = self.load(&name).await {
                    println!("⚠️ 加载插件失败 {}: {}", name, e);
                }
            }
        }

        Ok(())
    }

    /// 获取注册表
    pub fn registry(&self) -> Arc<PluginRegistry> {
        self.registry.clone()
    }

    /// 获取钩子管理器
    pub fn hook_manager(&self) -> Arc<HookManager> {
        self.hook_manager.clone()
    }

    /// 获取命令注册表
    pub fn command_registry(&self) -> Arc<CommandRegistry> {
        self.command_registry.clone()
    }

    /// 获取沙箱
    pub fn sandbox(&self) -> Arc<PluginSandbox> {
        self.sandbox.clone()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
