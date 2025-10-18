//! 提供教务网`my.cqu.edu.cn`的已知可用接口

use serde::{Deserialize, Serialize};
use snafu::{ensure, whatever};

use crate::{
    errors,
    errors::mycqu::MyCQUResult,
    mycqu::utils::{access::get_oauth_token, mycqu_request_handler},
    session::{Client, Session, access_info::MyCQUAccessInfo},
    sso::access_services,
    utils::{
        ApiModel,
        consts::{MYCQU_API_USER_URL, MYCQU_SERVICE_URL},
    },
};

pub mod course;
pub mod enroll;
pub mod exam;
pub mod score;
mod utils;

#[cfg(test)]
mod tests;

/// 获取访问教务网`my.cqu.edu.cn`的权限
pub async fn access_mycqu(client: &Client, session: &mut Session) -> MyCQUResult<()> {
    ensure!(session.is_login, errors::NotLoginSnafu {});

    // access_services 只会因为网络原因产生异常，不会产生任何`SSOError`
    whatever!(
        access_services(client, session, MYCQU_SERVICE_URL).await,
        "Unexpected SSOError happened"
    );

    let auth_token = get_oauth_token(client, session).await?;
    session.access_infos.mycqu_access_info = Some(MyCQUAccessInfo {
        auth_header: auth_token,
    });
    Ok(())
}

/// 教务网用户信息接口响应数据模型
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    /// 姓名
    pub name: String,
    /// 统一身份认证号
    #[serde(alias = "username")]
    pub id: String,
    /// 学工号
    pub code: String,
    /// 身份，已知取值有学生(`student`)、教师(`instructor`)
    #[serde(alias = "type")]
    pub role: String,
    /// 电子邮箱
    pub email: Option<String>,
    /// 电话号码
    #[serde(alias = "phoneNumber")]
    pub phone_number: Option<String>,
}

impl ApiModel for User {}

impl User {
    /// 通过具有教务网权限的会话([`Session`])，从教务网获取已登陆会话的用户信息([`User`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::mycqu::{access_mycqu, User};
    /// use rsmycqu::session::{Client, Session};
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_user() {
    /// # let client = Client::default();
    /// # let mut session = Session::new();
    /// login(&client, &mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&client, &mut session).await.unwrap();
    /// let user = User::fetch_self(&client, &session);
    /// # }
    /// ```
    pub async fn fetch_self(client: &Client, session: &Session) -> MyCQUResult<Self> {
        let res =
            mycqu_request_handler(client, session, |client| client.get(MYCQU_API_USER_URL)).await?;

        Ok(res.json::<Self>().await?)
    }
}
