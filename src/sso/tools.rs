use reqwest::{Response, StatusCode};

use crate::{
    errors::{
        ApiError,
        sso::{SSOError, SSOResult},
    },
    session::Session,
    sso::{encrypt::encrypt_password, logout},
    utils::{
        consts::SSO_LOGIN_URL,
        get_response_header,
        page_parser::{SSOLoginPageData, sso_login_parser},
    },
};

/// 登陆页面返回数据，根据该数据确定是否需要验证码或登陆链接
pub(super) enum LoginPageResponse {
    HasLogin { login_url: String },
    NormalLogin { login_page_data: SSOLoginPageData },
}

async fn launch_normal_login_result(res: Response) -> SSOResult<LoginPageResponse> {
    Ok(LoginPageResponse::NormalLogin {
        login_page_data: sso_login_parser(&res.text().await?).ok_or(ApiError::Website {
            msg: "Failed to parse login page".to_string(),
        })?,
    })
}

/// 获取登陆请求所需数据
pub(super) async fn get_login_request_data(
    session: &mut Session,
    force_relogin: bool,
) -> SSOResult<LoginPageResponse> {
    let res = session.client.get(SSO_LOGIN_URL).send().await?;
    match res.status() {
        StatusCode::FOUND => {
            if force_relogin {
                logout(session).await?;
                let local_res = session.client.get(SSO_LOGIN_URL).send().await?;
                return launch_normal_login_result(local_res).await;
            }

            let jump_url =
                get_response_header(&res, "Location").ok_or(ApiError::location_error())?;

            let login_url_res = session.client.get(jump_url).send().await?;

            Ok(LoginPageResponse::HasLogin {
                login_url: get_response_header(&login_url_res, "Location")
                    .ok_or(ApiError::location_error())?
                    .to_string(),
            })
        }
        StatusCode::OK => launch_normal_login_result(res).await,
        other => Err(SSOError::UnknownSSOError {
            msg: format!(
                "status code {} is got (302 expected) when sending login post, \
            but can not find the element span.login_auth_error#msg",
                other
            )
            .to_string(),
        }
        .into()),
    }
}

#[inline]
pub(super) fn launch_login_data(
    username: impl AsRef<str>,
    password: impl AsRef<str>,
    login_page_data: &SSOLoginPageData,
) -> SSOResult<[(&'static str, String); 7]> {
    Ok([
        ("username", username.as_ref().to_owned()),
        ("type", "UsernamePassword".to_string()),
        ("_eventId", "submit".to_string()),
        ("geolocation", "".to_string()),
        ("execution", login_page_data.login_page_flowkey.to_owned()),
        ("croypto", login_page_data.login_croypto.to_owned()),
        (
            "password",
            encrypt_password(&login_page_data.login_croypto, password)?,
        ),
    ])
}
