//! Settings 模块 - 从 mod.rs 重新导出
//!
//! 此文件仅用于满足模块系统的文件结构要求。
//! 实际的 Settings 结构体定义在 mod.rs 中。

// 从父模块重新导出所有设置相关的类型
pub use super::{Settings, MemorySettings, VoiceSettings, PluginSettings};
