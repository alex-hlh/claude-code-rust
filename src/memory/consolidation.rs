//! 记忆整合模块 - 记忆整合引擎（Dream 机制）
//!
//! 整合过程包括：
//! 1. 相似记忆合并
//! 2. 低价值记忆清理
//! 3. 洞察提取

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::{MemoryEntry, MemoryType};

/// 整合配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationConfig {
    /// 最大记忆数量
    pub max_memories: usize,
    /// 重要性阈值
    pub importance_threshold: f32,
    /// 年龄阈值（小时）
    pub age_threshold_hours: u64,
    /// 整合间隔（小时）
    pub consolidation_interval_hours: u64,
    /// 是否启用自动整合
    pub enable_auto_consolidation: bool,
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self {
            max_memories: 10000,
            importance_threshold: 0.3,
            age_threshold_hours: 24,
            consolidation_interval_hours: 6,
            enable_auto_consolidation: true,
        }
    }
}

/// 整合结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub memories_before: usize,
    pub memories_after: usize,
    pub memories_consolidated: usize,
    pub memories_removed: usize,
    pub insights_generated: usize,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// 整合引擎
pub struct ConsolidationEngine {
    config: ConsolidationConfig,
    last_consolidation: Option<DateTime<Utc>>,
}

impl ConsolidationEngine {
    /// 创建整合引擎
    pub fn new(config: ConsolidationConfig) -> Self {
        Self {
            config,
            last_consolidation: None,
        }
    }

    /// 执行整合
    pub async fn consolidate(&self, memories: &[MemoryEntry]) -> anyhow::Result<Vec<MemoryEntry>> {
        let start = std::time::Instant::now();
        let memories_before = memories.len();

        let mut consolidated = Vec::new();
        let mut to_merge: Vec<&MemoryEntry> = Vec::new();

        // 按重要性排序
        let mut sorted_memories: Vec<_> = memories.iter().collect();
        sorted_memories.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal));

        for memory in sorted_memories {
            if consolidated.len() >= self.config.max_memories {
                break;
            }

            if self.should_keep(memory) {
                if self.is_similar_to_any(memory, &consolidated) {
                    to_merge.push(memory);
                } else {
                    consolidated.push(memory.clone());
                }
            }
        }

        // 合并相似的记忆
        if !to_merge.is_empty() {
            let merged = self.merge_memories(&to_merge);
            consolidated.push(merged);
        }

        // 提取洞察
        let insights_generated = self.extract_insights(memories, &mut consolidated);

        println!("🧠 整合完成: {} -> {} 条记忆 (生成 {} 条洞察)",
            memories_before, consolidated.len(), insights_generated);

        Ok(consolidated)
    }

    /// 判断记忆是否应该保留
    fn should_keep(&self, memory: &MemoryEntry) -> bool {
        // 高重要性的记忆总是保留
        if memory.importance >= self.config.importance_threshold {
            return true;
        }

        // 新记忆保留
        let age = (Utc::now() - memory.timestamp).num_hours() as u64;
        if age < self.config.age_threshold_hours {
            return true;
        }

        // 知识和偏好类型保留
        matches!(memory.memory_type, MemoryType::Knowledge | MemoryType::Preference)
    }

    /// 判断记忆是否与已有记忆相似
    fn is_similar_to_any(&self, memory: &MemoryEntry, others: &[MemoryEntry]) -> bool {
        others.iter().any(|other| {
            self.calculate_similarity(memory, other) > 0.8
        })
    }

    /// 计算两个记忆的相似度（基于词集合的 Jaccard 相似度）
    fn calculate_similarity(&self, a: &MemoryEntry, b: &MemoryEntry) -> f32 {
        if a.memory_type != b.memory_type {
            return 0.0;
        }

        let a_words: std::collections::HashSet<&str> = a.content.split_whitespace().collect();
        let b_words: std::collections::HashSet<&str> = b.content.split_whitespace().collect();

        if a_words.is_empty() || b_words.is_empty() {
            return 0.0;
        }

        let intersection = a_words.intersection(&b_words).count();
        let union = a_words.union(&b_words).count();

        intersection as f32 / union as f32
    }

    /// 合并多个相似记忆
    fn merge_memories(&self, memories: &[&MemoryEntry]) -> MemoryEntry {
        let mut combined_content = String::new();
        let mut max_importance: f32 = 0.0;
        let mut all_tags: Vec<String> = Vec::new();

        for memory in memories {
            if !combined_content.is_empty() {
                combined_content.push('\n');
            }
            combined_content.push_str(&memory.content);
            max_importance = max_importance.max(memory.importance);
            all_tags.extend(memory.tags.clone());
        }

        all_tags.sort();
        all_tags.dedup();

        MemoryEntry::new(memories[0].memory_type.clone(), &combined_content)
            .with_importance(max_importance + 0.1)
            .with_tags(all_tags)
    }

    /// 从记忆中提取洞察
    fn extract_insights(&self, memories: &[MemoryEntry], consolidated: &mut Vec<MemoryEntry>) -> usize {
        let mut insights = 0;

        // 统计各类型记忆数量
        let mut type_counts: std::collections::HashMap<MemoryType, usize> = std::collections::HashMap::new();
        for memory in memories {
            *type_counts.entry(memory.memory_type.clone()).or_insert(0) += 1;
        }

        // 如果某类型记忆数量较多，生成洞察
        for (memory_type, count) in type_counts {
            if count >= 10 {
                let type_name = match memory_type {
                    MemoryType::Session => "会话",
                    MemoryType::Conversation => "对话",
                    MemoryType::Knowledge => "知识",
                    MemoryType::Preference => "偏好",
                    MemoryType::Task => "任务",
                    MemoryType::Error => "错误",
                    MemoryType::Insight => "洞察",
                };

                let insight_content = format!("检测到模式: 已记录 {} 条{}记忆。考虑回顾优化。", count, type_name);

                consolidated.push(
                    MemoryEntry::new(MemoryType::Insight, &insight_content)
                        .with_importance(0.7)
                );
                insights += 1;
            }
        }

        // 检测重复错误
        let error_memories: Vec<_> = memories.iter()
            .filter(|m| m.memory_type == MemoryType::Error)
            .collect();

        if error_memories.len() >= 3 {
            let error_patterns: Vec<String> = error_memories.iter()
                .take(5)
                .map(|m| m.content.chars().take(100).collect::<String>())
                .collect();

            consolidated.push(
                MemoryEntry::new(MemoryType::Insight, &format!(
                    "检测到重复错误: {} 个。最近: {}", error_memories.len(), error_patterns.join("; ")
                ))
                .with_importance(0.8)
            );
            insights += 1;
        }

        insights
    }

    /// 判断是否应该整合
    pub fn should_consolidate(&self, memory_count: usize) -> bool {
        memory_count >= self.config.max_memories
    }

    /// 获取上次整合时间
    pub fn last_consolidation(&self) -> Option<DateTime<Utc>> {
        self.last_consolidation
    }

    /// 获取配置
    pub fn config(&self) -> &ConsolidationConfig {
        &self.config
    }
}

impl Default for ConsolidationEngine {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
