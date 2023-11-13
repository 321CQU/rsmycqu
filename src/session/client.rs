//! new type of [`reqwest::Client`]

use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use reqwest::redirect::Policy;
use reqwest::{Client as reqwestClient, ClientBuilder};

use crate::errors::session::SessionResult;

/// [`reqwest::Client`]的`new type`
///
/// [`Client`]实现了`Deref`以支持像传递[`reqwest::Client`]一样传递[`Client`]的引用
///
/// [`rsmycqu`](crate)的正确运行要求[`reqwest::Client`]关闭自动跳转且启用`cookies`
///
/// [`Client`]的唯二构造方法`default`, `custom`保证了这一点
pub(crate) struct Client(reqwestClient);

impl Deref for Client {
    type Target = reqwestClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Client(self.0.clone())
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for Client {
    /// [`Client`]默认构建
    ///
    /// 该默认构建会构建包含以下性质的[`reqwest::Client`]
    ///
    /// - 禁用自动重定向 [redirect(Policy::none())](ClientBuilder::redirect)
    /// - 启用cookies store [cookie_store(true)](ClientBuilder::cookie_store)
    /// - 禁用网络代理 [no_proxy](ClientBuilder::no_proxy)
    fn default() -> Self {
        let req_client = reqwestClient::builder()
            .redirect(Policy::none())
            .cookie_store(true)
            .no_proxy()
            .build()
            .unwrap();
        Client(req_client)
    }
}

impl Client {
    /// 自定义构建方法
    ///
    /// # warning
    /// 为了保证库正确运行，在执行自定义构建后会调用
    ///
    /// - [redirect(Policy::none())](ClientBuilder::redirect)
    /// - [cookie_store(true)](ClientBuilder::cookie_store)
    ///
    /// 以确保自动重定向被禁用、cookies被启用
    pub(super) fn custom<F>(custom_builder: F) -> SessionResult<Self>
    where
        F: Fn(&mut ClientBuilder) + 'static,
    {
        let mut builder = reqwestClient::builder();
        custom_builder(&mut builder);
        let client = builder
            .redirect(Policy::none())
            .cookie_store(true)
            .build()?;

        Ok(Client(client))
    }
}
