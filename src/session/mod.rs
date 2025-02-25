//! 在[`reqwest::Client`]的基础上增加了额外的状态以保证库运行正确性

pub(crate) use client::Client;
use reqwest::ClientBuilder;

use crate::{errors::session::SessionError, session::access_info::AccessInfos};

pub mod access_info;
mod client;

/// 发起校园信息请求的会话
///
/// [`rsmycqu`](crate)要求传入的[`reqwest::Client`]禁用自动重定向并启用cookies
/// [`Session`]的`new`, `custom`方法保证了这一点
#[derive(Clone, Debug)]
pub struct Session {
    /// 包裹了[`reqwest::Client`]的[`Client`]
    pub(crate) client: Client,
    /// 是否已经通过[`sso`](crate::sso)登陆
    pub(crate) is_login: bool,

    pub(crate) access_infos: AccessInfos,
}

impl Session {
    /// [`Session`]默认构建
    ///
    /// 该默认构建会构建包含以下性质的[`reqwest::Client`]
    ///
    /// - 禁用自动重定向 [redirect(Policy::none())](ClientBuilder::redirect)
    /// - 禁用网络代理 [no_proxy](ClientBuilder::no_proxy)
    pub fn new() -> Self {
        Session {
            client: Client::default(),
            is_login: false,
            access_infos: AccessInfos::default(),
        }
    }

    /// [`Session`]自定义构建
    ///
    /// # warning
    /// 为了保证库正确运行，在执行自定义构建后会调用
    ///
    /// - [redirect(Policy::none())](ClientBuilder::redirect)
    ///
    /// 以确保自动重定向被禁用、cookies被启用
    pub fn custom<F>(custom_builder: F) -> Result<Self, SessionError>
    where
        F: Fn(&mut ClientBuilder) + 'static,
    {
        let client = Client::custom(custom_builder)?;

        Ok(Session {
            client,
            is_login: false,
            access_infos: AccessInfos::default(),
        })
    }
}

impl Session {
    /// 获取是否已经通过[`sso`](crate::sso)登录
    pub fn is_login(&self) -> bool {
        self.is_login
    }

    /// 获取登录信息
    pub fn access_infos(&self) -> &AccessInfos {
        &self.access_infos
    }

    /// 设置登录信息
    pub fn set_access_infos(self, infos: AccessInfos) -> Self {
        Session {
            access_infos: infos,
            ..self
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}
