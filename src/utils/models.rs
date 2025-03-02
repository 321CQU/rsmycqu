//! 包含部分公用的数据模型

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// 表示一个时间段的数据模型
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period {
    /// 时间段开始时间
    pub start: u8,
    /// 时间段结束时间
    pub end: u8,
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}
