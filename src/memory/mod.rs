//! 记忆模块 - 完整的记忆管理系统
//!
//! 包含以下子模块：
//! - session: 会话管理
//! - history: 历史追踪
//! - context: 上下文管理
//! - storage: 持久化存储
//! - consolidation: 记忆整合（Dream 机制）
//!
//! 记忆类型包括：会话、对话、知识、偏好、任务、错误、洞察

pub mod session;     // 会话管理
pub mod history;    // 历史追踪
pub mod context;    // 上下文管理
pub mod storage;    // 持久化存储
pub mod consolidation;  // 记忆整合引擎

pub use session::{SessionManager, Session, SessionInfo};
pub use history::{HistoryManager, HistoryEntry, HistoryFilter};
pub use context::{ContextManager, ContextWindow, ContextEntry};
pub use storage::{Storage, StorageBackend};
pub use consolidation::{ConsolidationEngine, ConsolidationConfig};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// 记忆条目 - 存储的单个记忆单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// 唯一 ID
    pub id: String,
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 记忆内容
    pub content: String,
    /// 创建时间
    pub timestamp: DateTime<Utc>,
    /// 重要性评分（0.0-1.0）
    pub importance: f32,
    /// 标签列表
    pub tags: Vec<String>,
    /// 元数据
    pub metadata: HashMap<String, serde_json::Value>,
    /// 向量嵌入（用于语义搜索）
    pub embedding: Option<Vec<f32>>,
}

impl MemoryEntry {
    /// 创建新的记忆条目
    pub fn new(memory_type: MemoryType, content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            memory_type,
            content: content.to_string(),
            timestamp: Utc::now(),
            importance: 0.5,
            tags: Vec::new(),
            metadata: HashMap::new(),
            embedding: None,
        }
    }

    /// 设置重要性评分
    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    /// 添加标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }
}

/// 记忆类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemoryType {
    Session,      // 会话记忆
    Conversation, // 对话记忆
    Knowledge,    // 知识记忆
    Preference,   // 偏好记忆
    Task,        // 任务记忆
    Error,       // 错误记忆
    Insight,     // 洞察记忆
}

/// 记忆系统状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatus {
    /// 总记忆数
    pub total_memories: usize,
    /// 会话数
    pub session_count: usize,
    /// 对话数
    pub conversation_count: usize,
    /// 知识记忆数
    pub knowledge_count: usize,
    /// 上次整合时间
    pub last_consolidation: Option<DateTime<Utc>>,
    /// 存储大小（字节）
    pub storage_size_bytes: u64,
}

/// 记忆管理器 - 记忆系统的主入口
///
/// 协调会话管理、历史管理、上下文管理、存储和整合等功能。
pub struct MemoryManager {
    sessions: Arc<SessionManager>,
    history: Arc<HistoryManager>,
    context: Arc<ContextManager>,
    storage: Arc<Storage>,
    consolidation: Arc<ConsolidationEngine>,
    memories: Arc<RwLock<Vec<MemoryEntry>>>,
}

impl MemoryManager {
    /// 创建新的记忆管理器
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let memory_path = home.join(".claude-code").join("memory");

        std::fs::create_dir_all(&memory_path).ok();

        Self {
            sessions: Arc::new(SessionManager::new()),
            history: Arc::new(HistoryManager::new()),
            context: Arc::new(ContextManager::new()),
            storage: Arc::new(Storage::new(memory_path)),
            consolidation: Arc::new(ConsolidationEngine::new(Default::default())),
            memories: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 获取记忆系统状态
    pub async fn status(&self) -> anyhow::Result<MemoryStatus> {
        let memories = self.memories.read().await;

        let storage_size = self.storage.size().await.unwrap_or(0);

        Ok(MemoryStatus {
            total_memories: memories.len(),
            session_count: memories.iter().filter(|m| m.memory_type == MemoryType::Session).count(),
            conversation_count: memories.iter().filter(|m| m.memory_type == MemoryType::Conversation).count(),
            knowledge_count: memories.iter().filter(|m| m.memory_type == MemoryType::Knowledge).count(),
            last_consolidation: None,
            storage_size_bytes: storage_size,
        })
    }

    /// 添加记忆
    pub async fn add_memory(&self, entry: MemoryEntry) -> anyhow::Result<()> {
        let mut memories = self.memories.write().await;
        memories.push(entry.clone());

        self.storage.save_memory(&entry).await?;

        Ok(())
    }

    /// 根据 ID 获取记忆
    pub async fn get_memory(&self, id: &str) -> Option<MemoryEntry> {
        let memories = self.memories.read().await;
        memories.iter().find(|m| m.id == id).cloned()
    }

    /// 搜索记忆
    pub async fn search_memories(&self, query: &str) -> Vec<MemoryEntry> {
        let query_lower = query.to_lowercase();
        let memories = self.memories.read().await;

        memories.iter()
            .filter(|m| {
                m.content.to_lowercase().contains(&query_lower) ||
                m.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    /// 按类型获取记忆
    pub async fn get_memories_by_type(&self, memory_type: MemoryType) -> Vec<MemoryEntry> {
        let memories = self.memories.read().await;
        memories.iter()
            .filter(|m| m.memory_type == memory_type)
            .cloned()
            .collect()
    }

    /// 获取重要记忆
    pub async fn get_important_memories(&self, threshold: f32) -> Vec<MemoryEntry> {
        let memories = self.memories.read().await;
        memories.iter()
            .filter(|m| m.importance >= threshold)
            .cloned()
            .collect()
    }

    /// 清空所有记忆
    pub async fn clear(&self) -> anyhow::Result<()> {
        let mut memories = self.memories.write().await;
        memories.clear();

        self.storage.clear().await?;

        Ok(())
    }

    /// 导出记忆到文件
    pub async fn export(&self, output: &PathBuf) -> anyhow::Result<()> {
        let memories = self.memories.read().await;
        let content = serde_json::to_string_pretty(&*memories)?;
        tokio::fs::write(output, content).await?;
        Ok(())
    }

    /// 从文件导入记忆
    pub async fn import(&self, input: &PathBuf) -> anyhow::Result<()> {
        let content = tokio::fs::read_to_string(input).await?;
        let imported: Vec<MemoryEntry> = serde_json::from_str(&content)?;

        let mut memories = self.memories.write().await;
        memories.extend(imported);

        Ok(())
    }

    /// 执行记忆整合（Dream 机制）
    ///
    /// 整合过程会：
    /// 1. 分析现有记忆
    /// 2. 合并相似的记忆
    /// 3. 提取洞察
    /// 4. 更新记忆重要性
    pub async fn consolidate(&self) -> anyhow::Result<()> {
        let memories = self.memories.read().await;
        let consolidated = self.consolidation.consolidate(&memories).await?;

        drop(memories);

        let mut memories = self.memories.write().await;
        *memories = consolidated;

        Ok(())
    }

    /// 从存储加载记忆
    pub async fn load(&self) -> anyhow::Result<()> {
        let memories = self.storage.load_all().await?;
        let mut mem = self.memories.write().await;
        *mem = memories;
        Ok(())
    }

    /// 保存记忆到存储
    pub async fn save(&self) -> anyhow::Result<()> {
        let memories = self.memories.read().await;
        self.storage.save_all(&memories).await
    }

    /// 获取会话管理器
    pub fn sessions(&self) -> Arc<SessionManager> {
        self.sessions.clone()
    }

    /// 获取历史管理器
    pub fn history(&self) -> Arc<HistoryManager> {
        self.history.clone()
    }

    /// 获取上下文管理器
    pub fn context(&self) -> Arc<ContextManager> {
        self.context.clone()
    }

    /// 获取存储后端
    pub fn storage(&self) -> Arc<Storage> {
        self.storage.clone()
    }

    /// 获取整合引擎
    pub fn consolidation(&self) -> Arc<ConsolidationEngine> {
        self.consolidation.clone()
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
