//! 各模块支持服务所需的登陆信息
//!

/// [`mycqu`](crate::mycqu)所需的登陆信息
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MyCQUAccessInfo {
    /// 该字符串应当添加进后续访问请求头的`Authorization`项中
    pub auth_header: String,
}
