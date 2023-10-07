//! 提供教务网`my.cqu.edu.cn`的已知可用接口

use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::errors::mycqu::MyCQUResult;
use crate::errors::{Error, ErrorHandler};
use crate::mycqu::utils::access::get_oauth_token;
use crate::session::access_info::{AccessInfoValue, MYCQU_ACCESS_INFO_KEY};
use crate::session::Session;
use crate::sso::access_services;
use crate::utils::consts::MYCQU_SERVICE_URL;

use crate::utils::APIModel;

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

/// 教务网用户信息接口响应数据模型
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// 姓名
    pub name: String,
    /// 统一身份认证号
    #[serde(alias="username")]
    pub id: String,
    /// 学工号
    pub code: String,
    /// 身份，已知取值有学生(`student`)、教师(`instructor`)
    #[serde(alias="type")]
    pub role: String,
    /// 电子邮箱
    pub email: Option<String>,
    /// 电话号码
    #[serde(alias="phoneNumber")]
    pub phone_number: Option<String>
}

impl APIModel for User{}

impl User {
    pub async fn call(session: &Session) -> MyCQUResult<Self> {
        if !session.can_access(&MYCQU_ACCESS_INFO_KEY) {
            return Err(Error::NotAccess)
        }

        let res = session.client.get("https://my.cqu.edu.cn/authserver/simple-user").send().await?;

        if res.status() == StatusCode::UNAUTHORIZED {
            return Err(Error::NotAccess)
        }
        Ok(res.json::<Self>().await?)
    }
}
