//! 宿舍水电费相关API
use reqwest::{
    StatusCode,
    header::{COOKIE, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use snafu::ensure;

use crate::{
    card::utils::card_request_handler,
    errors,
    errors::{ApiError, card::CardResult},
    session::{Client, Session},
    utils::{
        ApiModel,
        consts::{
            CARD_BLADE_AUTH_URL, CARD_GET_DORM_FEE_URL, CARD_PAGE_TICKET_POST_FORM_URL,
            CARD_PAGE_URL,
        },
    },
};

async fn get_page_ticket(client: &Client, session: &Session) -> CardResult<String> {
    let res = card_request_handler(client, session, |client| {
        client.post(CARD_PAGE_URL).form(&[
            ("EMenuName", "电费、网费"),
            ("MenuName", "电费、网费"),
            ("Url", CARD_PAGE_TICKET_POST_FORM_URL),
            ("apptype", "4"),
            ("flowID", "10002"),
        ])
    })
    .await?;

    ensure!(
        res.status() == StatusCode::OK,
        errors::WebsiteSnafu {
            msg: "Get Page Ticket Error".to_string()
        }
    );

    Ok(regex!("ticket=(.*)\'")
        .captures(&res.text().await?)
        .and_then(|item| item.get(1))
        .ok_or(ApiError::Website {
            msg: "Page Ticket Not Found".to_string(),
        })?
        .as_str()
        .to_string())
}

async fn get_synjones_auth(
    client: &Client,
    session: &Session,
    ticket: impl AsRef<str>,
) -> CardResult<String> {
    let res = card_request_handler(client, session, |client| {
        client
            .post(CARD_BLADE_AUTH_URL)
            .form(&[("ticket", ticket.as_ref()), ("json", "true")])
    })
    .await?;

    ensure!(
        res.status() == StatusCode::OK,
        errors::WebsiteSnafu {
            msg: "Get Synjones Auth Error".to_string()
        }
    );

    let data = res.json::<Map<String, Value>>().await?;
    let token = data
        .get("data")
        .and_then(|item| item.get("access_token"))
        .and_then(|item| item.as_str())
        .ok_or(ApiError::Website {
            msg: "Synjones Auth Token Not Found".to_string(),
        })?;

    Ok(format!("bearer {}", token))
}

/// 补助信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Subsidy {
    /// 虎溪校区补助信息
    Huxi {
        /// 电剩余补助
        #[serde(alias = "电剩余补助（度）")]
        electricity: String,
        /// 水剩余补助
        #[serde(alias = "水剩余补助（吨）")]
        water: String,
    },
    /// 老校区补助信息
    Old {
        /// 补贴余额
        #[serde(alias = "补贴余额")]
        subsidies: String,
    },
}

/// 某宿舍的水电费相关信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyFees {
    /// 账户余额
    #[serde(alias = "剩余金额")]
    #[serde(alias = "现金余额")]
    pub balance: String,
    /// 补助
    #[serde(flatten)]
    pub subsidies: Subsidy,
}

impl ApiModel for EnergyFees {}

impl EnergyFees {
    /// 通过具有校园卡查询网址权限的会话([`Session`])，获取宿舍水电费([`EnergyFees`])
    ///
    /// *会向会话中添加额外信息以加快后续相同API查询*
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::card::{access_card, EnergyFees};
    /// # use rsmycqu::session::{Client, Session};
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_dorm_fee() {
    /// # let client = Client::default();
    /// # let mut session = Session::new();
    /// login(&client, &mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_card(&client, &mut session).await.unwrap();
    /// let fees = EnergyFees::fetch_self(&client, &mut session, "b5321", true);
    /// # }
    /// ```
    pub async fn fetch_self(
        client: &Client,
        session: &mut Session,
        room: impl AsRef<str>,
        is_huxi: bool,
    ) -> CardResult<EnergyFees> {
        let card_access_info = session
            .access_infos
            .card_access_info
            .as_ref()
            .ok_or(ApiError::NotAccess)?;

        if card_access_info.synjones_auth.is_none() {
            let ticket = get_page_ticket(client, session).await?;
            let synjones_auth = get_synjones_auth(client, session, ticket).await?;
            session
                .access_infos
                .card_access_info
                .as_mut()
                .unwrap()
                .synjones_auth
                .replace(synjones_auth);
        }

        let synjones_auth = session
            .access_infos
            .card_access_info
            .as_ref()
            .unwrap()
            .synjones_auth
            .as_ref()
            .unwrap();
        let cookie_header = HeaderValue::from_str(&format!("synjones-auth={}", synjones_auth))
            .map_err(|err| ApiError::Website {
                msg: format!("Set cookies error: {}", err),
            })?;

        let res = card_request_handler(client, session, |client| {
            client
                .post(CARD_GET_DORM_FEE_URL)
                .form(&[
                    ("feeitemid", if is_huxi { "182" } else { "181" }), // 虎溪校区该项为'182'，老校区为'181'
                    ("json", "true"),
                    ("level", "2"),
                    ("room", room.as_ref()),
                    ("type", "IEC"),
                ])
                .header(COOKIE, cookie_header)
        })
        .await?;

        ensure!(
            res.status() == StatusCode::OK,
            errors::WebsiteSnafu {
                msg: "Get Dorm Fee Error".to_string()
            }
        );

        let mut json = res.json::<Map<String, Value>>().await?;

        let msg = json
            .get("msg")
            .and_then(|item| item.as_str())
            .ok_or(ApiError::Website {
                msg: "Website response format incorrect".to_string(),
            })?;
        ensure!(
            msg == "success",
            errors::WebsiteSnafu {
                msg: "Website response format incorrect".to_string()
            }
        );

        json.get_mut("map")
            .and_then(|item| item.get_mut("showData"))
            .map(Value::take)
            .map(serde_json::from_value)
            .ok_or(ApiError::Website {
                msg: "Website response format incorrect".to_string(),
            })?
            .map_err(|_| ApiError::ModelParse {
                msg: "Website response format incorrect".to_string(),
            })
    }
}
