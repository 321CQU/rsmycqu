//! [`errors`]模块提供了[`rsmycqu`]的所有错误定义

use snafu::Snafu;

use crate::errors::sealed::Sealed;

pub mod session;

#[cfg(feature = "sso")]
pub mod sso;

#[cfg(feature = "mycqu")]
pub mod mycqu;

#[cfg(feature = "card")]
pub mod card;

mod sealed {
    use crate::errors::sso;

    pub trait Sealed {}

    #[cfg(feature = "sso")]
    impl Sealed for sso::SSOError {}

    #[cfg(feature = "mycqu")]
    impl Sealed for crate::errors::mycqu::MyCQUError {}

    #[cfg(feature = "card")]
    impl Sealed for card::CardError {}
}

pub trait RsMyCQUError: std::error::Error + 'static + Sealed {}

/// 错误类型
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ApiError<T: RsMyCQUError> {
    /// 当用户未登陆时抛出
    #[snafu(display("Request Before Login"))]
    NotLogin,

    /// 当用户未获取相应服务访问权限时抛出
    #[snafu(display("Request Before Get Access"))]
    NotAccess,

    /// 当使用[reqwest]发起的请求失败时抛出
    #[snafu(display("Reqwest Error: {source}"))]
    Request {
        /// [reqwest]抛出的错误[reqwest::Error]
        source: reqwest::Error,
    },

    /// 数据模型解析异常时抛出
    #[snafu(display("Model Parse Error: {msg}"))]
    ModelParse {
        /// 错误信息
        msg: String,
    },

    /// 当请求网站出现异常时抛出
    #[snafu(display("Request Website Error: {msg}"))]
    Website {
        /// 错误详细信息
        msg: String,
    },

    /// 内部错误
    #[snafu(transparent)]
    Inner {
        /// 内部错误
        source: T,
    },

    /// 未知错误
    #[snafu(whatever)]
    Whatever {
        /// 错误信息
        message: String,
        /// 错误来源
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

#[allow(private_bounds)]
impl<T: RsMyCQUError> ApiError<T> {
    pub(crate) fn location_error() -> Self {
        ApiError::Website {
            msg: "Expected response has \"Location\" but not found".to_string(),
        }
    }
}

impl<T: RsMyCQUError> From<reqwest::Error> for ApiError<T> {
    fn from(source: reqwest::Error) -> Self {
        ApiError::Request { source }
    }
}

/// API返回结果
pub type ApiResult<T, E> = Result<T, ApiError<E>>;
