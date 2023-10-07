//! [rsmycqu] 的工具库

pub(crate) mod consts;
#[cfg(feature = "sso")]
pub(crate) mod encrypt;
#[cfg(feature = "sso")]
pub(crate) mod page_parser;

#[cfg(test)]
pub(crate) mod test_tools;
