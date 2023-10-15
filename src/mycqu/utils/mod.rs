use reqwest::{RequestBuilder, Response, StatusCode};
use crate::errors::Error;
use crate::errors::mycqu::MyCQUResult;
use crate::session::{Client, Session};

pub(super) mod access;

pub(super) async fn mycqu_request_handler<T>(session: &Session, f: T) -> MyCQUResult<Response> where T: FnOnce(&Client) -> RequestBuilder {
    if session.mycqu_access_info.is_none() {
        return Err(Error::NotAccess)
    }

    let res = f(&session.client)
        .bearer_auth(session.mycqu_access_info.as_ref().unwrap().auth_header.as_str())
        .send().await?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(Error::NotAccess)
    }
    Ok(res)
}
