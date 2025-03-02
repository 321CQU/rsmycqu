use reqwest::{RequestBuilder, Response, StatusCode};

use crate::{
    errors::{ApiError, card::CardResult},
    session::{Client, Session},
};

pub(super) async fn card_request_handler<T>(session: &Session, f: T) -> CardResult<Response>
where
    T: FnOnce(&Client) -> RequestBuilder,
{
    if session.access_infos.card_access_info.is_none() {
        return Err(ApiError::NotAccess);
    }

    let res = f(&session.client).send().await?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(ApiError::NotAccess);
    }
    Ok(res)
}
