//! 上下文管理 - 上下文窗口和 Token 管理
//!
//! 负责管理发送给 LLM 的上下文内容，包括：
//! - 自动 Token 计数和限制
//! - 优先级管理（关键上下文不会被清除）
//! - 上下文压缩和摘要

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 上下文条目 - 单条上下文消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEntry {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub token_count: usize,
    pub priority: ContextPriority,
    pub source: ContextSource,
}

impl ContextEntry {
    /// 创建新的上下文条目
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role: role.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            token_count: Self::estimate_tokens(content),
            priority: ContextPriority::Normal,
            source: ContextSource::User,
        }
    }

    /// 创建系统消息（高优先级）
    pub fn system(content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role: "system".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            token_count: Self::estimate_tokens(content),
            priority: ContextPriority::Critical,
            source: ContextSource::System,
        }
    }

    /// 创建助手消息
    pub fn assistant(content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role: "assistant".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            token_count: Self::estimate_tokens(content),
            priority: ContextPriority::Normal,
            source: ContextSource::Assistant,
        }
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: ContextPriority) -> Self {
        self.priority = priority;
        self
    }

    /// 估算 Token 数量（简单估计）
    fn estimate_tokens(text: &str) -> usize {
        text.split_whitespace().count() / 3 * 4
    }
}

/// 上下文优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContextPriority {
    Low,      // 低优先级（可被清除）
    Normal,   // 普通优先级
    High,     // 高优先级
    Critical, // 关键优先级（不会被清除）
}

/// 上下文来源
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextSource {
    System,   // 系统消息
    User,     // 用户消息
    Assistant, // 助手消息
    Tool,     // 工具返回
    Memory,   // 记忆
    File,     // 文件内容
}

/// 上下文窗口 - 管理有限长度的上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    /// 最大 Token 数
    pub max_tokens: usize,
    /// 保留 Token 数（系统留的 buffer）
    pub reserved_tokens: usize,
    /// 上下文条目队列
    pub entries: VecDeque<ContextEntry>,
    /// 当前总 Token 数
    pub total_tokens: usize,
}

impl ContextWindow {
    /// 创建新的上下文窗口
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            reserved_tokens: max_tokens / 10,
            entries: VecDeque::new(),
            total_tokens: 0,
        }
    }

    /// 获取可用 Token 数
    pub fn available_tokens(&self) -> usize {
        self.max_tokens.saturating_sub(self.reserved_tokens).saturating_sub(self.total_tokens)
    }

    /// 检查是否能容纳指定数量的 Token
    pub fn can_fit(&self, tokens: usize) -> bool {
        self.available_tokens() >= tokens
    }

    /// 添加上下文条目
    /// 如果空间不足，会自动清除低优先级条目
    pub fn add(&mut self, entry: ContextEntry) -> bool {
        if !self.can_fit(entry.token_count) {
            self.evict(entry.token_count);
        }

        if self.can_fit(entry.token_count) {
            self.total_tokens += entry.token_count;
            self.entries.push_back(entry);
            return true;
        }

        false
    }

    /// 清除条目以释放空间
    fn evict(&mut self, needed_tokens: usize) {
        let mut freed = 0;

        let mut to_remove = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            // 跳过关键优先级的条目
            if entry.priority == ContextPriority::Critical {
                continue;
            }

            if freed >= needed_tokens {
                break;
            }

            freed += entry.token_count;
            to_remove.push(i);
        }

        for i in to_remove.into_iter().rev() {
            if let Some(entry) = self.entries.remove(i) {
                self.total_tokens -= entry.token_count;
            }
        }
    }

    /// 清空上下文
    pub fn clear(&mut self) {
        self.entries.clear();
        self.total_tokens = 0;
    }

    /// 转换为消息列表
    pub fn to_messages(&self) -> Vec<crate::api::ChatMessage> {
        self.entries.iter()
            .map(|e| crate::api::ChatMessage {
                role: e.role.clone(),
                content: Some(e.content.clone()),
                tool_calls: None,
                tool_call_id: None,
            })
            .collect()
    }
}

/// 上下文管理器
pub struct ContextManager {
    window: Arc<RwLock<ContextWindow>>,
    summaries: Arc<RwLock<Vec<ContextSummary>>>,
}

/// 上下文摘要 - 压缩后的上下文记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSummary {
    pub id: String,
    pub summary: String,
    pub original_entries: usize,
    pub original_tokens: usize,
    pub created_at: DateTime<Utc>,
}

impl ContextManager {
    /// 创建新的上下文管理器（默认 128K token）
    pub fn new() -> Self {
        Self {
            window: Arc::new(RwLock::new(ContextWindow::new(128000))),
            summaries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 创建指定最大 Token 数的上下文管理器
    pub fn with_max_tokens(max_tokens: usize) -> Self {
        Self {
            window: Arc::new(RwLock::new(ContextWindow::new(max_tokens))),
            summaries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 添加上下文条目
    pub async fn add(&self, entry: ContextEntry) -> bool {
        let mut window = self.window.write().await;
        window.add(entry)
    }

    /// 添加用户消息
    pub async fn add_user(&self, content: &str) -> bool {
        self.add(ContextEntry::new("user", content)).await
    }

    /// 添加助手消息
    pub async fn add_assistant(&self, content: &str) -> bool {
        self.add(ContextEntry::assistant(content)).await
    }

    /// 添加系统消息
    pub async fn add_system(&self, content: &str) -> bool {
        self.add(ContextEntry::system(content)).await
    }

    /// 获取消息列表
    pub async fn get_messages(&self) -> Vec<crate::api::ChatMessage> {
        let window = self.window.read().await;
        window.to_messages()
    }

    /// 获取所有条目
    pub async fn get_entries(&self) -> Vec<ContextEntry> {
        let window = self.window.read().await;
        window.entries.iter().cloned().collect()
    }

    /// 清空上下文
    pub async fn clear(&self) {
        let mut window = self.window.write().await;
        window.clear();
    }

    /// 获取上下文统计
    pub async fn stats(&self) -> ContextStats {
        let window = self.window.read().await;
        ContextStats {
            total_entries: window.entries.len(),
            total_tokens: window.total_tokens,
            max_tokens: window.max_tokens,
            available_tokens: window.available_tokens(),
            utilization: window.total_tokens as f64 / window.max_tokens as f64,
        }
    }

    /// 创建上下文摘要
    pub async fn summarize(&self, summary: &str) -> ContextSummary {
        let window = self.window.read().await;

        let ctx_summary = ContextSummary {
            id: uuid::Uuid::new_v4().to_string(),
            summary: summary.to_string(),
            original_entries: window.entries.len(),
            original_tokens: window.total_tokens,
            created_at: Utc::now(),
        };

        let mut summaries = self.summaries.write().await;
        summaries.push(ctx_summary.clone());

        ctx_summary
    }

    /// 获取所有摘要
    pub async fn get_summaries(&self) -> Vec<ContextSummary> {
        let summaries = self.summaries.read().await;
        summaries.clone()
    }
}

/// 上下文统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStats {
    pub total_entries: usize,
    pub total_tokens: usize,
    pub max_tokens: usize,
    pub available_tokens: usize,
    pub utilization: f64,
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}
