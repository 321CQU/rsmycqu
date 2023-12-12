//! 提供校园卡网页(`card.cqu.edu.cn`)已知接口

use reqwest::StatusCode;
use crate::errors::card::{CardError, CardResult};
use crate::errors::{Error, ErrorHandler};
use crate::session::access_info::CardAccessInfo;
use crate::session::Session;
use crate::sso::access_services;
use crate::utils::consts::{CARD_HALL_TICKET_URL, CARD_SERVICE_URL};
use crate::utils::get_response_header;
use crate::utils::page_parser::card_access_parser;

pub use crate::card::dorm::*;
pub use crate::card::card::*;

mod dorm;
mod card;
mod utils;

#[cfg(test)]
mod tests;

/// 获取访问校园卡网站`card.cqu.edu.cn`的权限
pub async fn access_card(session: &mut Session) -> CardResult<()> {
    if !session.is_login {
        return Err(Error::NotLogin);
    }

    match access_services(&session.client, CARD_SERVICE_URL).await {
        // access_services 只会因为网络原因产生异常，不会产生任何`SSOError`
        Err(err) => {return Err(err.handle_other_error(|_| "Unexpected SSOError happened".into()));}
        Ok(res) => {
            let res = session.client.get(
                get_response_header(&res, "Location")
                    .ok_or::<Error<CardError>>("Expected response has \"Location\" but not found".into())?
            ).send().await?;
            let sso_ticket_id = card_access_parser(res.text().await?).map_err(|err| {
                Error::UnExceptedError {
                    msg: format!(
                        "Expected to successfully parse the page, but received: {}",
                        err
                    ),
                }
            })?;

            let res = session.client.post(CARD_HALL_TICKET_URL)
                .form(&[
                    ("errorcode", "1"),
                    ("ssoticketid", &sso_ticket_id),
                    ("continueurl", CARD_HALL_TICKET_URL),
                ])
                .send()
                .await?;

            if res.status() != StatusCode::OK && res.status() != StatusCode::FOUND {
                return Err(CardError::AccessError { msg: "Access Error".to_string() }.into())
            }
        }
    }

    session.card_access_info = Some(CardAccessInfo { synjones_auth: None });

    Ok(())
}


