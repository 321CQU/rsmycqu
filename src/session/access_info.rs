//! 各模块支持服务所需的登陆信息

use serde::{Deserialize, Serialize};

/// [`mycqu`](crate::mycqu)所需的登陆信息
#[cfg(feature = "mycqu")]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MyCQUAccessInfo {
    /// 该字符串应当添加进后续访问请求头的`Authorization`项中
    pub(crate) auth_header: String,
}

/// [`card`](crate::card)所需的登陆信息
#[cfg(feature = "card")]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CardAccessInfo {
    /// 用于宿舍水电费查询的账号信息
    pub(crate) synjones_auth: Option<String>,
}

/// 包含各模块支持服务所需的登陆信息
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct AccessInfos {
    #[cfg(feature = "mycqu")]
    pub(crate) mycqu_access_info: Option<MyCQUAccessInfo>,
    #[cfg(feature = "card")]
    pub(crate) card_access_info: Option<CardAccessInfo>,
}
