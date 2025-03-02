//! 图书馆相关查询接口

use serde::{Deserialize, Serialize};

/// 图书馆书籍相关信息
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookInfo {
    /// 书籍id
    pub id: Option<u32>,
    /// 书籍名称
    pub title: String,
    /// 书籍检索号
    pub call_no: String,
    /// 所属图书馆（如虎溪图书馆自然科学阅览室等）
    pub library_name: String,
    /// 借出时间
    pub borrow_time: String,
    /// 应归还日期
    pub should_return_time: Option<String>,
    /// 是否被归还
    pub is_return: bool,
    /// 归还时间
    pub return_time: Option<String>,
    /// 续借次数
    pub renew_count: u16,
    /// 是否可被续借
    pub can_renew: bool,
}
