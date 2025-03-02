//! new type of [`reqwest::Client`]

use std::{fmt::Debug, ops::Deref};

use reqwest::{redirect::Policy, ClientBuilder};

use crate::errors::session::SessionError;

/// [`reqwest::Client`]的`new type`
///
/// [`Client`]实现了`Deref`以支持像传递[`reqwest::Client`]一样传递[`Client`]的引用
///
/// [`rsmycqu`](crate)的正确运行要求[`reqwest::Client`]关闭自动跳转
///
/// [`Client`]的唯二构造方法`default`, `custom`保证了这一点
#[derive(Clone, Debug)]
pub(crate) struct Client(reqwest::Client);

impl Default for Client {
    /// [`Client`]默认构建
    ///
    /// 该默认构建会构建包含以下性质的[`reqwest::Client`]
    ///
    /// - 禁用自动重定向 [redirect(Policy::none())](ClientBuilder::redirect)
    /// - 禁用网络代理 [no_proxy](ClientBuilder::no_proxy)
    fn default() -> Self {
        let req_client = reqwest::Client::builder()
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
    ///
    /// 以确保自动重定向被禁用
    pub(super) fn custom<F>(custom_builder: F) -> Result<Self, SessionError>
    where
        F: Fn(&mut ClientBuilder),
    {
        let mut builder = reqwest::Client::builder();
        custom_builder(&mut builder);
        let client = builder
            .redirect(Policy::none())
            .cookie_store(true)
            .build()?;

        Ok(Client(client))
    }
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
