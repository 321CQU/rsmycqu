//! 包含会在[`crate::mycqu`]中出现的所有错误

use crate::errors::{Error, PubInnerError};
use snafu::prelude::*;
use std::fmt::Debug;

/// MyCQUError
#[derive(Debug, Snafu)]
pub enum MyCQUError {
    /// 获取`my.cqu.edu.cn`服务访问权限错误时抛出
    #[snafu(display("{msg}"))]
    AccessError {
        /// 错误详细信息
        msg: String,
    },
}

/// [Result<T, Error<MyCQUError>>]的重命名
pub type MyCQUResult<T> = Result<T, Error<MyCQUError>>;

impl PubInnerError for MyCQUError {}
