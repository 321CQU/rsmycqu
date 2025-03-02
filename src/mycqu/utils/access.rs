use std::collections::HashMap;

use serde_json::Value;

use crate::{
    errors::mycqu::{MyCQUError, MyCQUResult},
    session::Client,
    utils::{
        consts::{MYCQU_AUTHORIZE_URL, MYCQU_TOKEN_INDEX_URL, MYCQU_TOKEN_URL},
        get_response_header,
    },
};

#[inline]
fn find_code(location: &(impl AsRef<str> + ?Sized)) -> MyCQUResult<&str> {
    Ok(regex!(r"\?code=([^&]+)&")
        .captures(location.as_ref())
        .and_then(|captures| captures.get(1))
        .ok_or(MyCQUError::AccessError)?
        .as_str())
}

pub(in crate::mycqu) async fn get_oauth_token(client: &Client) -> MyCQUResult<String> {
    let res = client.get(MYCQU_AUTHORIZE_URL).send().await?;
    let code = find_code(get_response_header(&res, "Location").ok_or(MyCQUError::AccessError)?)?;
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
        .ok_or(MyCQUError::AccessError)?
        .as_str()
        .map(ToString::to_string)
        .ok_or(MyCQUError::AccessError.into())
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
