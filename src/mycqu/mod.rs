//! 提供教务网`my.cqu.edu.cn`的已知可用接口

use crate::errors::mycqu::MyCQUResult;
use crate::errors::{Error, ErrorHandler};
use crate::mycqu::utils::access::get_oauth_token;
use crate::session::access_info::{AccessInfoValue, MYCQU_ACCESS_INFO_KEY};
use crate::session::Session;
use crate::sso::access_services;
use crate::utils::consts::MYCQU_SERVICE_URL;

mod utils;

#[cfg(test)]
mod tests;

/// 获取访问教务网`my.cqu.edu.cn`的权限
pub async fn access_mycqu(session: &mut Session) -> MyCQUResult<()> {
    if !session.is_login {
        return Err(Error::NotLogin);
    }

    // access_services 只会因为网络原因产生异常，不会产生任何`SSOError`
    if let Err(err) = access_services(&session.client, MYCQU_SERVICE_URL).await {
        return Err(err.handle_other_error(|_| Error::UnExceptedError {
            msg: "Except no SSOError happened".to_string(),
        }));
    }

    let auth_token = get_oauth_token(&session.client).await?;
    session.access_info.insert(
        &MYCQU_ACCESS_INFO_KEY,
        AccessInfoValue::MyCQU {
            auth_header: auth_token,
        },
    );
    Ok(())
}
