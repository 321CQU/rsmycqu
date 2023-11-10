//! [`errors`]模块提供了[`rsmycqu`]的所有错误定义

use std::error::Error as stdError;

use snafu::Snafu;

#[cfg(feature = "sso")]
pub use sso::*;

pub mod session;
#[cfg(feature = "sso")]
pub(crate) mod page_parser;
#[cfg(feature = "sso")]
pub mod sso;

#[cfg(feature = "mycqu")]
pub mod mycqu;

pub(crate) trait PubInnerError: stdError {}

/// 支持不同泛型的[`Error`]相互转换
pub(crate) trait ErrorHandler<T: PubInnerError> {
    fn handle_other_error<U: PubInnerError, F>(self, inner_error_handler: F) -> Error<U>
        where
            F: Fn(T) -> Error<U>;
}

/// 错误类型
#[derive(Debug, Snafu)]
pub enum Error<T: stdError> {
    /// 当用户未登陆时抛出
    #[snafu(display("Request Before Login"))]
    NotLogin,

    /// 当用户未获取相应服务访问权限时抛出
    #[snafu(display("Request Before Get Access"))]
    NotAccess,

    /// 预期外的错误
    #[snafu(display("{msg}"))]
    UnExceptedError {
        /// 该错误如何不符合预期
        msg: String,
    },

    /// 当使用[reqwest]发起的请求失败时抛出
    #[snafu(display("Reqwest Error: {err}"))]
    RequestError {
        /// [reqwest]抛出的错误[reqwest::Error]
        err: reqwest::Error,
    },

    /// 数据模型解析异常时抛出
    #[snafu(display("Model Parse Error"))]
    ModelParseError,

    /// 其他错误
    #[snafu(display("{err}"))]
    InnerError {
        /// 引发的具体错误
        err: T,
    },
}

pub(crate) mod error_handle_help {
    use crate::errors::{Error, ErrorHandler, PubInnerError};

    impl<T: PubInnerError> From<T> for Error<T> {
        fn from(value: T) -> Self {
            Error::InnerError { err: value }
        }
    }

    impl<T: PubInnerError> From<reqwest::Error> for Error<T> {
        fn from(value: reqwest::Error) -> Self {
            Error::RequestError { err: value }
        }
    }

    impl<T: PubInnerError> From<reqwest::header::ToStrError> for Error<T> {
        fn from(value: reqwest::header::ToStrError) -> Self {
            Error::UnExceptedError {
                msg: format!(
                    "Expected http header can transfer to str, but received: {}",
                    value
                ),
            }
        }
    }

    impl<T: PubInnerError> ErrorHandler<T> for Error<T> {
        fn handle_other_error<U: PubInnerError, F>(self, inner_error_handler: F) -> Error<U>
            where
                F: Fn(T) -> Error<U>,
        {
            match self {
                Error::NotLogin => Error::NotLogin,
                Error::NotAccess => Error::NotAccess,
                Error::UnExceptedError { msg } => Error::UnExceptedError { msg },
                Error::RequestError { err } => Error::RequestError { err },
                Error::ModelParseError => Error::ModelParseError,
                Error::InnerError { err: inner_err } => inner_error_handler(inner_err),
            }
        }
    }
}
