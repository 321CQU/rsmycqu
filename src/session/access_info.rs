//! 各模块支持服务所需的登陆状态

/// 用于访问服务所需登陆状态的键
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct AccessInfoKey {
    id: &'static str,
}

/// 用于访问[`mycqu`](crate::mycqu)所需登陆状态的键
pub const MYCQU_ACCESS_INFO_KEY: AccessInfoKey = AccessInfoKey { id: "mycqu" };

/// 各模块支持服务所需的登陆状态
#[derive(Debug, Clone)]
pub enum AccessInfoValue {
    /// [`mycqu`](crate::mycqu)所需的登陆状态
    MyCQU {
        /// 该字符串应当添加进后续访问请求头的`Authorization`项中
        auth_header: String,
    },
}
