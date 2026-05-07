//! 历史记录管理 - 命令和查询的历史追踪
//!
//! 记录所有执行的命令、查询、工具调用等操作，
//! 支持按类型、时间、会话筛选和搜索。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 历史记录条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub entry_type: HistoryType,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<String>,
    pub success: bool,
    pub duration_ms: Option<u64>,
    pub metadata: serde_json::Value,
}

impl HistoryEntry {
    /// 创建新的历史记录
    pub fn new(entry_type: HistoryType, content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entry_type,
            content: content.to_string(),
            timestamp: Utc::now(),
            session_id: None,
            success: true,
            duration_ms: None,
            metadata: serde_json::Value::Null,
        }
    }

    /// 设置关联的会话 ID
    pub fn with_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    /// 设置执行时长
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    /// 设置成功状态
    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }
}

/// 历史记录类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HistoryType {
    Command,     // 命令
    Query,       // 查询
    ToolCall,    // 工具调用
    FileOperation, // 文件操作
    Search,      // 搜索
    Agent,       // Agent 执行
}

/// 历史记录筛选器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFilter {
    pub entry_type: Option<HistoryType>,
    pub session_id: Option<String>,
    pub success_only: bool,
    pub from_time: Option<DateTime<Utc>>,
    pub to_time: Option<DateTime<Utc>>,
    pub limit: usize,
}

impl Default for HistoryFilter {
    fn default() -> Self {
        Self {
            entry_type: None,
            session_id: None,
            success_only: false,
            from_time: None,
            to_time: None,
            limit: 100,
        }
    }
}

/// 历史记录管理器
pub struct HistoryManager {
    entries: Arc<RwLock<VecDeque<HistoryEntry>>>,
    history_path: PathBuf,
    max_entries: usize,
}

impl HistoryManager {
    /// 创建新的历史记录管理器
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let history_path = home.join(".claude-code").join("history.json");

        Self {
            entries: Arc::new(RwLock::new(VecDeque::new())),
            history_path,
            max_entries: 10000,
        }
    }

    /// 添加历史记录
    pub async fn add(&self, entry: HistoryEntry) -> anyhow::Result<()> {
        let mut entries = self.entries.write().await;

        // 超过最大条数时移除最旧的记录
        if entries.len() >= self.max_entries {
            entries.pop_front();
        }

        entries.push_back(entry);
        self.save(&entries).await?;

        Ok(())
    }

    /// 获取指定 ID 的记录
    pub async fn get(&self, id: &str) -> Option<HistoryEntry> {
        let entries = self.entries.read().await;
        entries.iter().find(|e| e.id == id).cloned()
    }

    /// 按筛选条件列出记录
    pub async fn list(&self, filter: HistoryFilter) -> Vec<HistoryEntry> {
        let entries = self.entries.read().await;

        let mut result: Vec<HistoryEntry> = entries.iter()
            .filter(|e| {
                if let Some(ref entry_type) = filter.entry_type {
                    if e.entry_type != *entry_type {
                        return false;
                    }
                }

                if let Some(ref session_id) = filter.session_id {
                    if e.session_id.as_ref() != Some(session_id) {
                        return false;
                    }
                }

                if filter.success_only && !e.success {
                    return false;
                }

                if let Some(from) = filter.from_time {
                    if e.timestamp < from {
                        return false;
                    }
                }

                if let Some(to) = filter.to_time {
                    if e.timestamp > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        result.truncate(filter.limit);
        result
    }

    /// 搜索历史记录
    pub async fn search(&self, query: &str) -> Vec<HistoryEntry> {
        let query_lower = query.to_lowercase();
        let entries = self.entries.read().await;

        entries.iter()
            .filter(|e| e.content.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    /// 获取最近 N 条记录
    pub async fn get_recent(&self, count: usize) -> Vec<HistoryEntry> {
        let entries = self.entries.read().await;
        entries.iter().rev().take(count).cloned().collect()
    }

    /// 按类型获取记录
    pub async fn get_by_type(&self, entry_type: HistoryType, limit: usize) -> Vec<HistoryEntry> {
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|e| e.entry_type == entry_type)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// 清空历史记录
    pub async fn clear(&self) -> anyhow::Result<()> {
        let mut entries = self.entries.write().await;
        entries.clear();
        self.save(&entries).await
    }

    /// 获取历史统计
    pub async fn stats(&self) -> HistoryStats {
        let entries = self.entries.read().await;

        let mut commands = 0;
        let mut queries = 0;
        let mut tool_calls = 0;
        let mut successful = 0;
        let mut failed = 0;

        for entry in entries.iter() {
            match entry.entry_type {
                HistoryType::Command => commands += 1,
                HistoryType::Query => queries += 1,
                HistoryType::ToolCall => tool_calls += 1,
                _ => {}
            }

            if entry.success {
                successful += 1;
            } else {
                failed += 1;
            }
        }

        HistoryStats {
            total_entries: entries.len(),
            commands,
            queries,
            tool_calls,
            successful,
            failed,
        }
    }

    /// 保存到磁盘
    async fn save(&self, entries: &VecDeque<HistoryEntry>) -> anyhow::Result<()> {
        if let Some(parent) = self.history_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(entries)?;
        tokio::fs::write(&self.history_path, content).await?;

        Ok(())
    }

    /// 从磁盘加载
    pub async fn load(&self) -> anyhow::Result<()> {
        if !self.history_path.exists() {
            return Ok(());
        }

        let content = tokio::fs::read_to_string(&self.history_path).await?;
        let loaded: VecDeque<HistoryEntry> = serde_json::from_str(&content)?;

        let mut entries = self.entries.write().await;
        *entries = loaded;

        Ok(())
    }
}

/// 历史统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_entries: usize,
    pub commands: usize,
    pub queries: usize,
    pub tool_calls: usize,
    pub successful: usize,
    pub failed: usize,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}
