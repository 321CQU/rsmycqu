//! [rsmycqu] 的工具库

pub use models::*;

pub trait APIModel {}

pub mod models;

pub(crate) mod consts;

#[cfg(feature = "mycqu")]
pub(crate) mod datetimes;
#[cfg(feature = "sso")]
pub(crate) mod encrypt;
#[cfg(feature = "sso")]
pub(crate) mod page_parser;

#[cfg(test)]
pub(crate) mod test_fixture;
