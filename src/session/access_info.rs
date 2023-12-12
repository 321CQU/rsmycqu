//! 各模块支持服务所需的登陆信息


/// [`mycqu`](crate::mycqu)所需的登陆信息
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MyCQUAccessInfo {
    /// 该字符串应当添加进后续访问请求头的`Authorization`项中
    pub auth_header: String,
}

/// [`card`](crate::card)所需的登陆信息
#[cfg(feature = "card")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardAccessInfo {
    /// 用于宿舍水电费查询的账号信息
    pub synjones_auth: Option<String>,
}
