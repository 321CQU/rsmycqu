use reqwest::{RequestBuilder, Response, StatusCode};
use crate::errors::card::CardResult;
use crate::errors::Error;
use crate::session::{Client, Session};


pub(super) async fn card_request_handler<T>(session: &Session, f: T) -> CardResult<Response>
    where
        T: FnOnce(&Client) -> RequestBuilder,
{
    if session.card_access_info.is_none() {
        return Err(Error::NotAccess);
    }

    let res = f(&session.client)
        .send()
        .await?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(Error::NotAccess);
    }
    Ok(res)
}