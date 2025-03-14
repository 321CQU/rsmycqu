//! 重庆大学单点登录（SSO）相关模块

use reqwest::{Response, StatusCode};
use snafu::ensure;

use crate::{
    errors,
    errors::{
        ApiError,
        sso::{SSOError, SSOResult},
    },
    session::{Client, Session},
    sso::tools::{LoginPageResponse, get_login_request_data, launch_login_data},
    utils::{
        consts::{SSO_LOGIN_URL, SSO_LOGOUT_URL},
        get_response_header,
    },
};

mod encrypt;
mod tools;

#[cfg(test)]
mod tests;

/// 退出账号登陆
pub async fn logout(session: &mut Session) -> SSOResult<()> {
    let client = &session.client;
    client
        .get(SSO_LOGOUT_URL)
        .send()
        .await
        .map_err(|_| SSOError::LogoutError)?;
    session.is_login = false;

    Ok(())
}

/// 登陆结果
#[derive(Debug, Eq, PartialEq)]
pub enum LoginResult {
    /// 登陆成功
    Success,

    /// 账号或密码错误
    IncorrectLoginCredentials,
}

/// 登陆账号
pub async fn login(
    session: &mut Session,
    auth: impl AsRef<str>,
    password: impl AsRef<str>,
    force_relogin: bool,
) -> SSOResult<LoginResult> {
    let request_data = get_login_request_data(session, force_relogin).await?;

    match request_data {
        LoginPageResponse::HasLogin { login_url } => {
            session.client.get(login_url).send().await?;
            session.is_login = true;
            Ok(LoginResult::Success)
        }

        LoginPageResponse::NormalLogin { login_page_data } => {
            let login_data = launch_login_data(auth, password, &login_page_data)?;
            let res = session
                .client
                .post(SSO_LOGIN_URL)
                .form(&login_data)
                .send()
                .await?;

            match res.status() {
                StatusCode::FOUND => {
                    let url =
                        get_response_header(&res, "Location").ok_or(ApiError::location_error())?;
                    session.client.get(url).send().await?;
                    session.is_login = true;
                    Ok(LoginResult::Success)
                }
                StatusCode::UNAUTHORIZED => Ok(LoginResult::IncorrectLoginCredentials),
                other => Err(SSOError::UnknownSSOError {
                    msg: format!("Unexpected status code: {}", other),
                }
                .into()),
            }
        }
    }
}

#[cfg(feature = "mycqu")]
/// 使用登陆了统一身份认证的账号获取指定服务许可
pub(super) async fn access_services(
    client: &Client,
    service: impl AsRef<str>,
) -> SSOResult<Response> {
    let res = client
        .get(SSO_LOGIN_URL)
        .query(&[("service", service.as_ref())])
        .send()
        .await?;

    ensure!(res.status() == StatusCode::FOUND, errors::NotLoginSnafu {});

    let jump_url = get_response_header(&res, "Location").ok_or(ApiError::location_error())?;

    Ok(client.get(jump_url).send().await?)
}
