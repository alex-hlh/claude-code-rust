//! 会话管理 - 会话生命周期管理
//!
//! 会话是指用户在一次工作会话中的完整交互记录。
//! 每个会话包含多条消息、关联的项目路径和元数据。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 会话结构体 - 表示一次完整的用户工作会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// 会话唯一 ID
    pub id: String,
    /// 会话名称（可选，用于人类识别）
    pub name: String,
    /// 关联的项目路径
    pub project_path: Option<PathBuf>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 会话中的消息列表
    pub messages: Vec<SessionMessage>,
    /// 自定义元数据
    pub metadata: HashMap<String, serde_json::Value>,
    /// 会话状态
    pub status: SessionStatus,
}

impl Session {
    /// 创建新会话
    pub fn new(name: Option<&str>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            name: name.unwrap_or(&id).to_string(),
            project_path: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            messages: Vec::new(),
            metadata: HashMap::new(),
            status: SessionStatus::Active,
        }
    }

    /// 设置关联项目路径
    pub fn with_project(mut self, path: PathBuf) -> Self {
        self.project_path = Some(path);
        self
    }

    /// 添加消息到会话
    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(SessionMessage {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        });
        self.updated_at = Utc::now();
    }

    /// 获取消息数量
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

/// 会话消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessage {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 会话状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Active,   // 活跃
    Paused,   // 暂停
    Archived, // 已归档
    Error,    // 错误
}

/// 会话摘要信息（用于列表展示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub project_path: Option<PathBuf>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub status: SessionStatus,
}

/// 会话管理器 - 管理所有会话的创建、加载、保存和删除
pub struct SessionManager {
    sessions_dir: PathBuf,
    active_session: Arc<RwLock<Option<Session>>>,
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionManager {
    /// 创建新的会话管理器
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let sessions_dir = home.join(".claude-code").join("sessions");

        Self {
            sessions_dir,
            active_session: Arc::new(RwLock::new(None)),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 创建新会话
    pub async fn create(&self, name: Option<&str>) -> anyhow::Result<Session> {
        let session = Session::new(name);
        self.save(&session).await?;

        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    /// 加载会话
    pub async fn load(&self, id: &str) -> anyhow::Result<Option<Session>> {
        let path = self.sessions_dir.join(format!("{}.json", id));

        if !path.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(&path).await?;
        let session: Session = serde_json::from_str(&content)?;

        let mut sessions = self.sessions.write().await;
        sessions.insert(id.to_string(), session.clone());

        Ok(Some(session))
    }

    /// 保存会话到磁盘
    pub async fn save(&self, session: &Session) -> anyhow::Result<()> {
        tokio::fs::create_dir_all(&self.sessions_dir).await?;

        let path = self.sessions_dir.join(format!("{}.json", session.id));
        let content = serde_json::to_string_pretty(session)?;
        tokio::fs::write(&path, content).await?;

        Ok(())
    }

    /// 删除会话
    pub async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let path = self.sessions_dir.join(format!("{}.json", id));

        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }

        let mut sessions = self.sessions.write().await;
        sessions.remove(id);

        Ok(())
    }

    /// 列出所有会话
    pub async fn list(&self) -> anyhow::Result<Vec<SessionInfo>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.values().map(|s| SessionInfo {
            id: s.id.clone(),
            name: s.name.clone(),
            project_path: s.project_path.clone(),
            created_at: s.created_at,
            updated_at: s.updated_at,
            message_count: s.messages.len(),
            status: s.status.clone(),
        }).collect())
    }

    /// 获取指定会话
    pub async fn get(&self, id: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(id).cloned()
    }

    /// 设置当前活跃会话
    pub async fn set_active(&self, session: Session) {
        let mut active = self.active_session.write().await;
        *active = Some(session);
    }

    /// 获取当前活跃会话
    pub async fn get_active(&self) -> Option<Session> {
        let active = self.active_session.read().await;
        active.clone()
    }

    /// 清空活跃会话
    pub async fn clear_active(&self) {
        let mut active = self.active_session.write().await;
        *active = None;
    }

    /// 添加消息到会话
    pub async fn add_message(&self, id: &str, role: &str, content: &str) -> anyhow::Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(id) {
            session.add_message(role, content);
            self.save(session).await?;
        }
        Ok(())
    }

    /// 归档会话
    pub async fn archive(&self, id: &str) -> anyhow::Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(id) {
            session.status = SessionStatus::Archived;
            self.save(session).await?;
        }
        Ok(())
    }

    /// 搜索会话
    pub async fn search(&self, query: &str) -> Vec<SessionInfo> {
        let query_lower = query.to_lowercase();
        let sessions = self.sessions.read().await;

        sessions.values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower) ||
                s.messages.iter().any(|m| m.content.to_lowercase().contains(&query_lower))
            })
            .map(|s| SessionInfo {
                id: s.id.clone(),
                name: s.name.clone(),
                project_path: s.project_path.clone(),
                created_at: s.created_at,
                updated_at: s.updated_at,
                message_count: s.messages.len(),
                status: s.status.clone(),
            })
            .collect()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
