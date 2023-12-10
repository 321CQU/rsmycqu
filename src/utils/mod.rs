//! [rsmycqu] 的工具库

use reqwest::header::AsHeaderName;
use reqwest::Response;
pub use macros::*;
pub use models::*;

pub trait APIModel {}

pub mod models;

pub(crate) mod consts;
#[macro_use]
pub(crate) mod macros;

#[cfg(feature = "mycqu")]
pub(crate) mod datetimes;
#[cfg(feature = "sso")]
pub(crate) mod encrypt;
#[cfg(feature = "sso")]
pub(crate) mod page_parser;

#[cfg(test)]
pub(crate) mod test_fixture;

#[inline]
pub(crate) fn get_response_header(res: &Response, target: impl AsHeaderName) -> Option<&str> {
    res
        .headers()
        .get(target)
        .and_then(|value| value.to_str().ok())
}
