//! [rsmycqu](crate)是重庆大学校园信息查询库`pymycqu`的rust版本，完全支持`pymycqu`中所有已经支持的API
//!
//! 目前已经实现以下功能
//! - 所有下列接口响应的数据模型
//! - [重庆大学单点登陆(SSO)](sso)
//! - [重庆大学教务网相关功能](mycqu)
//!     - [获取访问教务网API权限](mycqu::access_mycqu)
//!

#![warn(missing_docs)]

mod utils;

pub mod errors;
#[cfg(feature = "mycqu")]
pub mod mycqu;
pub mod session;
#[cfg(feature = "sso")]
pub mod sso;
