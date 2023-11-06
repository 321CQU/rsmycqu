//! 提供教务网`my.cqu.edu.cn`的已知可用接口

use serde::{Serialize, Deserialize};

use crate::errors::mycqu::MyCQUResult;
use crate::errors::{Error, ErrorHandler};
use crate::mycqu::utils::access::get_oauth_token;
use crate::session::Session;
use crate::sso::access_services;
use crate::utils::consts::{MYCQU_API_USER_URL, MYCQU_SERVICE_URL};
use crate::mycqu::utils::mycqu_request_handler;
use crate::session::access_info::MyCQUAccessInfo;
use crate::utils::APIModel;

pub use course::*;
pub use score::*;
pub use exam::*;

mod utils;
pub mod course;
pub mod score;
pub mod exam;

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
            msg: "Unexpected SSOError happened".to_string(),
        }));
    }

    let auth_token = get_oauth_token(&session.client).await?;
    session.mycqu_access_info = Some(MyCQUAccessInfo{ auth_header: auth_token });
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
    /// 通过具有教务网权限的会话([`Session`])，从教务网获取已登陆会话的用户信息([`User`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::mycqu::{access_mycqu, User};
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_user() {
    /// let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = User::fetch_self(&session);
    /// # }
    /// ```
    pub async fn fetch_self(session: &Session) -> MyCQUResult<Self> {
        let res = mycqu_request_handler(session, |client| client.get(MYCQU_API_USER_URL)).await?;

        Ok(res.json::<Self>().await?)
    }
}
