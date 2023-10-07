use crate::errors::sso::{SSOError, SSOResult};
use crate::errors::Error;
use crate::session::Session;
use crate::sso::encrypt::encrypt_password;
use crate::sso::logout;
use crate::utils::{
    consts::SSO_LOGIN_URL,
    page_parser::sso_parser::{sso_login_parser, SSOLoginPageData},
};
use reqwest::{Response, StatusCode};

/// 登陆页面返回数据，根据该数据确定是否需要验证码或登陆链接
pub(super) enum LoginPageResponse {
    HasLogin { login_url: String },
    NormalLogin { login_page_data: SSOLoginPageData },
}

async fn launch_normal_login_result(res: Response) -> SSOResult<LoginPageResponse> {
    Ok(LoginPageResponse::NormalLogin {
        login_page_data: sso_login_parser(&res.text().await?).map_err(
            |err|
                Error::UnExceptedError {
                    msg: format!("Expected to successfully parse the page, but received: {}", err)
                }
        )?,
    })
}

/// 获取登陆请求所需数据
pub(super) async fn get_login_request_data(
    session: &mut Session,
    force_relogin: &bool,
) -> SSOResult<LoginPageResponse> {
    let res = session.client.get(SSO_LOGIN_URL).send().await?;
    match res.status() {
        StatusCode::FOUND => {
            if *force_relogin {
                logout(session).await?;
                let local_res = session.client.get(SSO_LOGIN_URL).send().await?;
                return launch_normal_login_result(local_res).await;
            }

            let jump_url = res
                .headers()
                .get("Location")
                .ok_or(Error::UnExceptedError {
                    msg: "Expected response has \"Location\" but not found".to_string(),
                })?
                .to_str()?;

            let login_url_res = session.client.get(jump_url).send().await?;

            Ok(LoginPageResponse::HasLogin {
                login_url: login_url_res
                    .headers()
                    .get("Location")
                    .ok_or(Error::UnExceptedError {
                        msg: "Expected response has \"Location\" but not found".to_string(),
                    })?
                    .to_str()?
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
    username: &str,
    password: &str,
    login_page_data: &SSOLoginPageData,
) -> SSOResult<[(&'static str, String); 7]> {
    Ok([
        ("username", username.to_owned()),
        ("type", "UsernamePassword".to_string()),
        ("_eventId", "submit".to_string()),
        ("geolocation", "".to_string()),
        ("execution", login_page_data.login_page_flowkey.to_owned()),
        ("croypto", login_page_data.login_croypto.to_owned()),
        ("password", encrypt_password(&login_page_data.login_croypto, password)?),
    ])
}
