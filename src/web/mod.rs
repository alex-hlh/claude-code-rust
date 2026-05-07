//! Web 模块 - 插件市场 Web 界面
//!
//! 本模块使用 Axum 框架提供 Web 服务器用于插件市场。

pub mod server;
pub mod routes;
pub mod handlers;
pub mod models;
pub mod templates;

pub use server::WebServer;
pub use models::*;
