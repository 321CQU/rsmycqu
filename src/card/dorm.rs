//! 宿舍水电费相关API
use reqwest::header::{COOKIE, HeaderValue};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::card::utils::card_request_handler;
use crate::errors::card::{CardError, CardResult};
use crate::errors::Error;
use crate::session::Session;
use crate::session::access_info::CardAccessInfo;
use crate::utils::consts::{CARD_BLADE_AUTH_URL, CARD_GET_DORM_FEE_URL, CARD_PAGE_TICKET_POST_FORM_URL, CARD_PAGE_URL};

async fn get_page_ticket(session: &Session) -> CardResult<String> {
    let res = card_request_handler(session, |client| {
        client.post(CARD_PAGE_URL)
            .form(&[
                ("EMenuName", "电费、网费"),
                ("MenuName", "电费、网费"),
                ("Url", CARD_PAGE_TICKET_POST_FORM_URL),
                ("apptype", "4"),
                ("flowID", "10002")
            ])
    }).await?;

    if res.status() != StatusCode::OK {
        return Err(CardError::WebsiteError {msg: "Get Page Ticket Error".to_string()}.into());
    }

    Ok(
        regex!("ticket=(.*)\'")
            .captures(&res.text().await?)
            .and_then(|item| item.get(1))
            .ok_or(Error::UnExceptedError {msg: "Page Ticket Not Found".to_string()})?
            .as_str()
            .to_string()
    )
}

async fn get_synjones_auth(session: &Session, ticket: impl AsRef<str>) -> CardResult<String> {
    let res = card_request_handler(session, |client| {
        client.post(CARD_BLADE_AUTH_URL)
            .form(&[
                ("ticket", ticket.as_ref()),
                ("json", "true"),
            ])
    }).await?;

    if res.status() != StatusCode::OK {
        return Err(CardError::WebsiteError {msg: "Get Synjones Auth Error".to_string()}.into());
    }

    let data = res.json::<Map<String, Value>>().await?;
    let token = data.get("data")
        .and_then(|item| item.get("access_token"))
        .and_then(|item| item.as_str())
        .ok_or(Error::UnExceptedError {msg: "Synjones Auth Token Not Found".to_string()})?;

    Ok(
        format!("bearer {}", token)
    )
}

/// 某宿舍的水电费相关信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyFees {
    /// 账户余额
    pub balance: String,
    /// 电剩余补助（仅虎溪校区拥有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electricity_subsidy: Option<String>,
    /// 水剩余补助（仅虎溪校区拥有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_subsidy: Option<String>,
    /// 补助余额（仅老校区拥有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidies: Option<String>,
}

impl EnergyFees {
    /// 从json字典中解析[`EnergyFees`]
    pub(crate) fn from_json(json_map: &Map<String, Value>, is_huxi: bool) -> Option<Self> {
        let balance = json_map.get(if is_huxi { "剩余金额" } else { "现金余额" }).and_then(Value::as_str).map(ToString::to_string)?;

        Some(
            if is_huxi {
                EnergyFees {
                    balance,
                    electricity_subsidy: json_map.get("电剩余补助").and_then(Value::as_str).map(ToString::to_string),
                    water_subsidy: json_map.get("水剩余补助").and_then(Value::as_str).map(ToString::to_string),
                    subsidies: None,
                }
            } else {
                EnergyFees {
                    balance,
                    electricity_subsidy: None,
                    water_subsidy: None,
                    subsidies: json_map.get("补贴余额").and_then(Value::as_str).map(ToString::to_string),
                }
            }
        )
    }

    /// 通过具有校园卡查询网址权限的会话([`Session`])，获取宿舍水电费([`EnergyFees`])
    ///
    /// *会向会话中添加额外信息以加快后续相同API查询*
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::card::{access_card, EnergyFees};
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_dorm_fee() {
    /// let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_card(&mut session).await.unwrap();
    /// let fees = EnergyFees::fetch_self(&mut session, "b5321", true);
    /// # }
    /// ```
    pub async fn fetch_self(session: &mut Session, room: impl AsRef<str>, is_huxi: bool) -> CardResult<EnergyFees> {
        if session.card_access_info.is_none() { return Err(Error::NotAccess) }

        if session.card_access_info.as_ref().unwrap().synjones_auth.is_none() {
            let ticket = get_page_ticket(session).await?;
            let synjones_auth = get_synjones_auth(session, ticket).await?;
            session.card_access_info = Some(CardAccessInfo {synjones_auth: Some(synjones_auth)});
        }

        let synjones_auth = session.card_access_info.as_ref().unwrap().synjones_auth.as_ref().unwrap();
        let cookie_header = HeaderValue::from_str(&format!("synjones-auth={}", synjones_auth))
            .map_err(|err| Error::UnExceptedError {msg: format!("Set cookies error: {}", err)})?;

        let res = card_request_handler(session, |client| {
            client.post(CARD_GET_DORM_FEE_URL)
                .form(&[
                    ("feeitemid", if is_huxi { "182" } else { "181" }), // 虎溪校区该项为'182'，老校区为'181'
                    ("json", "true"),
                    ("level", "2"),
                    ("room", room.as_ref()),
                    ("type", "IEC"),
                ])
                .header(
                    COOKIE,
                    cookie_header
                )
        }).await?;

        if res.status() != StatusCode::OK {
            return Err(CardError::WebsiteError {msg: "Get Dorm Fee Error".to_string()}.into());
        }

        let json = res.json::<Map<String, Value>>().await?;

        if let Some(Value::String(msg)) =  json.get("msg") {
            if msg != "success" {
                return Err(CardError::WebsiteError {msg: msg.to_string()}.into())
            }

            if let Some(Value::Object(data)) = json.get("map").and_then(|item| item.get("showData")) {
                return EnergyFees::from_json(data, is_huxi).ok_or(Error::UnExceptedError {msg: "Website response format incorrect".to_string()})
            }
        }

        Err(Error::UnExceptedError {msg: "Website response format incorrect".to_string()})
    }
}
