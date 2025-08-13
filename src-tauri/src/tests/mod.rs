// src-tauri/src/tests/mod.rs
// 测试模块

pub mod basic_test;
// pub mod integration_test;  // 备份掉有编码问题的旧文件
pub mod integration_test_simple;

// 测试辅助函数和工具
pub use basic_test::*;
