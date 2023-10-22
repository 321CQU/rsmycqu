//! 包含部分公用的数据模型

use serde::{Deserialize, Serialize};

/// 表示一个时间段的数据模型
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period {
    /// 时间段开始时间
    pub start: u8,
    /// 时间段结束时间
    pub end: u8,
}