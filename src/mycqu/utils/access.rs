use crate::errors::mycqu::{MyCQUError, MyCQUResult};
use crate::session::Client;
use crate::utils::consts::{MYCQU_AUTHORIZE_URL, MYCQU_TOKEN_INDEX_URL, MYCQU_TOKEN_URL};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

#[inline]
fn find_code(location: &str) -> MyCQUResult<&str> {
    Ok(
        Regex::new(r"\?code=([^&]+)&").unwrap()
            .captures(location)
            .and_then(|captures| captures.get(1))
            .ok_or(MyCQUError::AccessError {msg: "Get Auth Code Error".to_string()})?
            .as_str()
    )
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
    if let Value::String(access_token) =
        access_res
            .get("access_token")
            .ok_or(MyCQUError::AccessError {
                msg: "Get Access Token Error".to_string(),
            })?
    {
        Ok(access_token.to_string())
    } else {
        Err(MyCQUError::AccessError {
            msg: "Get Access Token Error".to_string(),
        }
        .into())
    }
}

#[cfg(test)]
mod test {
    use super::find_code;
    use rstest::*;

    #[rstest]
    fn test_parse_code() {
        let location = "https://my.cqu.edu.cn/enroll/token-index?code=ZbfCVZ&state=";
        assert_eq!(find_code(location).unwrap(), "ZbfCVZ")
    }
}
