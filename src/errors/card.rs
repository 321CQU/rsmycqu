//! 包含会在[`crate::card`]中出现的所有错误

use std::fmt::Debug;

use snafu::prelude::*;

use crate::errors::{ApiResult, RsMyCQUError};

/// MyCQUError
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum CardError {
    /// 获取校园卡信息失败
    #[snafu(display("获取访问权限失败"))]
    AccessError,
}

impl RsMyCQUError for CardError {}

/// CardResult
pub type CardResult<T> = ApiResult<T, CardError>;
