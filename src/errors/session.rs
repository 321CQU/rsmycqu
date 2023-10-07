//! 包含会在[`crate::session`]中出现的所有错误

use crate::errors::{Error, PubInnerError};
use snafu::prelude::*;
use std::fmt::Debug;

/// SessionError
#[derive(Debug, Snafu)]
pub enum SessionError {
    /// 当加密密码错误时抛出
    #[snafu(display("Session Build Error: {err}"))]
    SessionBuildError{
        /// 引发该异常的[`reqwest::Error`]
        err: reqwest::Error
    }
}

/// [Result<T, Error<SessionError>>]的重命名
pub type SessionResult<T> = Result<T, Error<SessionError>>;

impl PubInnerError for SessionError {}

impl From<reqwest::Error> for SessionError {
    fn from(value: reqwest::Error) -> Self {
        Self::SessionBuildError {err: value}
    }
}