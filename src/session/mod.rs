//! 在[`reqwest::Client`]的基础上增加了额外的状态以保证库运行正确性

use reqwest::ClientBuilder;
use std::collections::HashMap;

use crate::session::access_info::AccessInfoKey;
use access_info::AccessInfoValue;
#[doc(inline)]
pub(crate) use client::Client;
#[doc(inline)]
pub use session_builder::*;

pub mod access_info;
mod client;
mod session_builder;

/// 发起校园信息请求的会话
///
/// [`rsmycqu`](crate)要求传入的[`reqwest::Client`]禁用自动重定向并启用cookies
/// [`Session`]的`new`, `custom`方法保证了这一点
#[derive(Clone)]
pub struct Session {
    /// 包裹了[`reqwest::Client`]的[`Client`]
    pub(crate) client: Client,
    /// 是否已经通过[`sso`](crate::sso)登陆
    pub is_login: bool,
    /// 是否已经通过[`mycqu`](crate::mycqu)获取`my.cqu.edu.cn`的访问权限
    pub access_info: HashMap<AccessInfoKey, AccessInfoValue>,
}

impl Session {
    /// [`Session`]默认构建
    ///
    /// 该默认构建会构建包含以下性质的[`reqwest::Client`]
    ///
    /// - 禁用自动重定向 [redirect(Policy::none())](ClientBuilder::redirect)
    /// - 启用cookies store [cookie_store(true)](ClientBuilder::cookie_store)
    /// - 禁用网络代理 [no_proxy](ClientBuilder::no_proxy)
    pub fn new() -> Self {
        Session {
            client: Client::default(),
            is_login: false,
            access_info: HashMap::new(),
        }
    }

    /// [`Session`]自定义构建
    ///
    /// # warning
    /// 为了保证库正确运行，在执行自定义构建后会调用
    ///
    /// - [redirect(Policy::none())](ClientBuilder::redirect)
    /// - [cookie_store(true)](ClientBuilder::cookie_store)
    ///
    /// 以确保自动重定向被禁用、cookies被启用
    pub fn custom<F>(custom_builder: F) -> Self
    where
        F: Fn(&mut ClientBuilder) + 'static,
    {
        let client = Client::custom(custom_builder);
        Session {
            client,
            is_login: false,
            access_info: HashMap::new(),
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}
