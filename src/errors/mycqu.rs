//! 包含会在[`crate::mycqu`]中出现的所有错误

use std::fmt::Debug;

use snafu::prelude::*;

use crate::errors::ApiResult;

/// MyCQUError
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum MyCQUError {
    #[snafu(display("获取访问权限失败"))]
    /// 获取访问权限失败
    AccessError,
}

impl crate::errors::RsMyCQUError for MyCQUError {}

/// MyCQUResult
pub type MyCQUResult<T> = ApiResult<T, MyCQUError>;
