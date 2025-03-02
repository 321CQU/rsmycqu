//! 提供校园卡网页(`card.cqu.edu.cn`)已知接口

use reqwest::StatusCode;
use snafu::{OptionExt, ensure, whatever};

pub use crate::card::{card::*, dorm::*};
use crate::{
    errors,
    errors::{
        ApiError,
        card::{CardError, CardResult},
    },
    session::{Session, access_info::CardAccessInfo},
    sso::access_services,
    utils::{
        consts::{CARD_HALL_TICKET_URL, CARD_SERVICE_URL},
        get_response_header,
        page_parser::card_access_parser,
    },
};

mod card;
mod dorm;
mod utils;

#[cfg(test)]
mod tests;

/// 获取访问校园卡网站`card.cqu.edu.cn`的权限
pub async fn access_card(session: &mut Session) -> CardResult<()> {
    ensure!(session.is_login, errors::NotLoginSnafu);

    let res = whatever!(
        access_services(&session.client, CARD_SERVICE_URL).await,
        "Unexpected SSOError happened"
    );

    let res = session
        .client
        .get(
            get_response_header(&res, "Location").ok_or(ApiError::ModelParse {
                msg: "Expected response has \"Location\" but not found".into(),
            })?,
        )
        .send()
        .await?;
    let sso_ticket_id = card_access_parser(res.text().await?)
        .whatever_context::<&str, ApiError<CardError>>("Unable to parse card page")?;

    let res = session
        .client
        .post(CARD_HALL_TICKET_URL)
        .form(&[
            ("errorcode", "1"),
            ("ssoticketid", &sso_ticket_id),
            ("continueurl", CARD_HALL_TICKET_URL),
        ])
        .send()
        .await?;

    ensure!(
        res.status() == StatusCode::OK || res.status() == StatusCode::FOUND,
        errors::card::AccessSnafu
    );

    session.access_infos.card_access_info = Some(CardAccessInfo {
        synjones_auth: None,
    });

    Ok(())
}
