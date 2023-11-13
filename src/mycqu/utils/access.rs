use std::collections::HashMap;

use regex::Regex;
use serde_json::Value;

use crate::errors::mycqu::MyCQUError;
use crate::errors::mycqu::MyCQUResult;
use crate::session::Client;
use crate::utils::consts::{MYCQU_AUTHORIZE_URL, MYCQU_TOKEN_INDEX_URL, MYCQU_TOKEN_URL};

#[inline]
fn find_code(location: &(impl AsRef<str> + ?Sized)) -> MyCQUResult<&str> {
    Ok(Regex::new(r"\?code=([^&]+)&")
        .unwrap()
        .captures(location.as_ref())
        .and_then(|captures| captures.get(1))
        .ok_or(MyCQUError::AccessError {
            msg: "Get Auth Code Error".to_string(),
        })?
        .as_str())
}

pub(in crate::mycqu) async fn get_oauth_token(client: &Client) -> MyCQUResult<String> {
    let res = client.get(MYCQU_AUTHORIZE_URL).send().await?;
    let code = find_code(
        res.headers()
            .get("Location")
            .ok_or(MyCQUError::AccessError {
                msg: "Get Auth Code Error".to_string(),
            })?
            .to_str()?,
    )?;
    let token_data = [
        ("client_id", "enroll-prod"),
        ("client_secret", "app-a-1234"),
        ("code", code),
        ("redirect_uri", MYCQU_TOKEN_INDEX_URL),
        ("grant_type", "authorization_code"),
    ];

    let access_res = client
        .post(MYCQU_TOKEN_URL)
        .form(&token_data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    access_res
        .get("access_token")
        .ok_or(MyCQUError::AccessError {
            msg: "Get Access Token Error".to_string(),
        })?
        .as_str()
        .map(ToString::to_string)
        .ok_or(
            MyCQUError::AccessError {
                msg: "Get Access Token Error".to_string(),
            }
            .into(),
        )
}

#[cfg(test)]
mod test {
    use rstest::*;

    use super::find_code;

    #[rstest]
    fn test_parse_code() {
        let location = "https://my.cqu.edu.cn/enroll/token-index?code=ZbfCVZ&state=";
        assert_eq!(find_code(location).unwrap(), "ZbfCVZ")
    }
}
