use reqwest::{RequestBuilder, Response, StatusCode};
use serde_json::{Map, Value};
use snafu::ensure;

use crate::{
    errors,
    errors::{ApiError, mycqu::MyCQUResult},
    session::{Client, Session},
};

pub(super) mod access;
pub(super) mod encrypt;

pub(super) async fn mycqu_request_handler<T>(session: &Session, f: T) -> MyCQUResult<Response>
where
    T: FnOnce(&Client) -> RequestBuilder,
{
    if session.access_infos.mycqu_access_info.is_none() {
        return Err(ApiError::NotAccess);
    }

    let res = f(&session.client)
        .bearer_auth(
            session
                .access_infos
                .mycqu_access_info
                .as_ref()
                .unwrap()
                .auth_header
                .as_str(),
        )
        .send()
        .await?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(ApiError::NotAccess);
    }
    Ok(res)
}

/// 检查响应json的status字段是否为error，如果是则返回错误
pub(super) fn check_website_response(res: &Map<String, Value>) -> MyCQUResult<()> {
    ensure!(
        res.get("status")
            .and_then(Value::as_str)
            .is_some_and(|status| status == "success"),
        errors::WebsiteSnafu {
            msg: res
                .get("msg")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .unwrap_or("".to_string())
        }
    );

    Ok(())
}
