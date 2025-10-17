//! 在[`reqwest::Client`]的基础上增加了额外的状态以保证库运行正确性

use std::sync::Arc;

pub use client::Client;
pub use reqwest;
pub(crate) use reqwest::{
    cookie::{CookieStore, Jar},
    header,
};

use crate::{errors::session::SessionError, session::access_info::AccessInfos};

pub mod access_info;
mod client;

/// 发起校园信息请求的会话
///
/// [`rsmycqu`](crate)要求传入的[`reqwest::Client`]禁用自动重定向并启用cookies
/// [`Session`]的`new`, `custom`方法保证了这一点
#[derive(Clone, Debug)]
pub struct Session {
    cookie_jar: Arc<Jar>,
    /// 是否已经通过[`sso`](crate::sso)登陆
    pub(crate) is_login: bool,
    /// 登陆后获取到的各服务访问信息
    pub(crate) access_infos: AccessInfos,
}

impl Session {
    /// 创建一个新的 `Session` 实例.
    ///
    /// # Arguments
    /// * `client` - 一个对共享的 `reqwest::Client` 实例的引用。
    ///   该共享客户端应在服务启动时创建一次，并应禁用重定向。
    pub fn new() -> Self {
        Session {
            cookie_jar: Arc::new(Jar::default()),
            is_login: false,
            access_infos: AccessInfos::default(),
        }
    }

    /// 使用此会话的上下文（特别是Cookie）来执行一个HTTP请求。
    ///
    /// 这个方法是与外界交互的核心。它会：
    /// 1. 从会话的 `cookie_jar` 中提取相关Cookie，并添加到请求头。
    /// 2. 发送请求。
    /// 3. 从响应中提取 `Set-Cookie` 头，并更新 `cookie_jar`。
    ///
    /// # Arguments
    /// * `builder` - 一个由共享客户端创建的 `reqwest::RequestBuilder`。
    pub async fn execute(
        &self,
        mut builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, SessionError> {
        let builder_cloned = builder.try_clone().unwrap().build()?;

        // 1. 从 Jar 中提取 cookies 并格式化为请求头字符串
        let cookie_header = self.cookie_jar.cookies(builder_cloned.url());

        // 2. 将 Cookie 头添加到请求中
        if let Some(cookie_header) = cookie_header {
            builder = builder.header(header::COOKIE, cookie_header)
        };

        // 3. 发送请求
        let response = builder.send().await?;

        // 4. 从响应中提取 Set-Cookie 并更新 Jar
        let response_url = response.url().clone();
        let headers = response.headers().clone();
        for cookie_str in headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
        {
            self.cookie_jar.add_cookie_str(cookie_str, &response_url);
        }

        Ok(response)
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
