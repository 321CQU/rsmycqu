//! 包含会在[`crate::sso`]中出现的所有错误

use crate::errors::{Error, PubInnerError};
use snafu::prelude::*;
use std::fmt::Debug;

/// SSOError
#[derive(Debug, Snafu)]
pub enum SSOError {
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