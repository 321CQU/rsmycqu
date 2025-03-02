//! 包含会在[`crate::sso`]中出现的所有错误

use std::fmt::Debug;

use snafu::prelude::*;

use crate::errors::{ApiResult, RsMyCQUError};

/// SSOError
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum SSOError {
    /// 当加密密码错误时抛出
    #[snafu(display("Password Encrypt Error"))]
    PasswordEncryptError {
        /// 错误信息
        source: base64::DecodeError,
    },

    /// 当登出失败时抛出
    #[snafu(display("Logout Error"))]
    LogoutError,

    /// 由教务网引发的错误
    #[snafu(display("{msg}"))]
    UnknownSSOError {
        /// 教务网的预期返回和实际返回
        msg: String,
    },
}

impl RsMyCQUError for SSOError {}

/// SSOResult
pub type SSOResult<T> = ApiResult<T, SSOError>;
