//! 包含会在[`crate::sso`]中出现的所有错误

use std::fmt::Debug;

use base64::DecodeError;
use snafu::prelude::*;

use crate::errors::{Error, PubInnerError};

/// SSOError
#[derive(Debug, Snafu)]
pub enum SSOError {
    /// 当加密密码错误时抛出
    #[snafu(display("Password Encrypt Error"))]
    PasswordEncryptError,

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

/// [Result<T, Error<SSOError>>]的重命名
pub type SSOResult<T> = Result<T, Error<SSOError>>;

impl PubInnerError for SSOError {}

impl From<DecodeError> for SSOError {
    fn from(_: DecodeError) -> Self {
        SSOError::PasswordEncryptError
    }
}
