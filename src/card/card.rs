//! 校园课余额、账单查询接口


use reqwest::header::CONTENT_LENGTH;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map, Value};
use crate::card::utils::card_request_handler;
use crate::errors::card::{CardError, CardResult};
use crate::errors::Error;
use crate::session::Session;
use crate::utils::consts::{CARD_GET_BILL_URL, CARD_GET_CARD_URL};

/// 校园卡相关信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Card {
    /// 校园卡id
    pub id: String,
    /// 账户余额
    pub amount: f64,
}

/// 校园卡账单相关信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Bill {
    /// 交易名称
    pub name: String,
    /// 交易时间
    pub date: String,
    /// 交易地点
    pub place: String,
    /// 交易金额
    pub tran_amount: f64,
    /// 账户余额
    pub acc_amount: f64
}

impl Card {
    /// 从json字典中解析[`Card`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        if let (Some(id), Some(amount)) = (
            json_map.get("acctNo").and_then(Value::as_number).map(ToString::to_string),
            json_map.get("acctAmt").and_then(Value::as_f64).map(|item| item / 100.0)
        ) {
            return Some(
                Card {
                    id,
                    amount,
                }
            )
        }

        None
    }

    /// 通过具有校园卡查询网址权限的会话([`Session`])，获取校园卡信息([`Card`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::card::{access_card, Card};
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_card() {
    /// let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_card(&mut session).await.unwrap();
    /// let card = Card::fetch_self(&mut session).await.unwrap();
    /// # }
    /// ```
    pub async fn fetch_self(session: &Session) -> CardResult<Card> {
        let res = card_request_handler(session, |client| {
            client.post(CARD_GET_CARD_URL)
                .header(CONTENT_LENGTH, 0)
        }).await?;

        let text = res.json::<String>().await?;
        let json = from_str::<Map<String, Value>>(&text)
            .map_err(|err| {
                println!("{:?}", err);
                Error::UnExceptedError {msg: "Website response format incorrect".to_string()}
            })?;

        if json.get("respCode").and_then(Value::as_str).is_some_and(|code| code != "0000") {
            return Err(
                CardError::WebsiteError {
                    msg: json
                        .get("respInfo")
                        .and_then(Value::as_str)
                        .unwrap_or("No Website Error")
                        .to_string()
                }.into()
            )
        }

        if let Some(Value::Object(data)) = json.get("objs").and_then(Value::as_array).and_then(|array| array.get(0)) {
            return Card::from_json(data).ok_or(Error::UnExceptedError {msg: "Website response format incorrect".to_string()})
        }

        Err(Error::UnExceptedError {msg: "Website response format incorrect".to_string()})
    }
}

impl Bill {
    /// 从json字典中解析[`Bill`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        if let (Some(name), Some(date), Some(place), Some(tran_amount), Some(acc_amount)) = (
            json_map.get("tranName").and_then(Value::as_str).map(ToString::to_string),
            json_map.get("tranDt").and_then(Value::as_str).map(ToString::to_string),
            json_map.get("mchAcctName").and_then(Value::as_str).map(ToString::to_string),
            json_map.get("tranAmt").and_then(Value::as_f64).map(|item| item / 100.0),
            json_map.get("acctAmt").and_then(Value::as_str).and_then(|item| item.parse::<f64>().ok()).map(|item| item / 100.0)
        ) {
            return Some(
                Bill {
                    name,
                    date,
                    place,
                    tran_amount,
                    acc_amount,
                }
            )
        }

        None
    }
}

impl Card {
    /// 通过具有校园卡查询网址权限的会话([`Session`])，获取校园卡账单信息([`Bill`])
    ///
    /// *`start_date`, `end_date`日期格式应当符合`%Y-%m-%d`*
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::card::{access_card, Card};
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_bill() {
    /// let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_card(&mut session).await.unwrap();
    /// let card = Card::fetch_self(&mut session).await.unwrap();
    /// let bills = card.fetch_bill(&session, "2023-11-10", "2023-12-12", 1, 100).await.unwrap();
    /// # }
    /// ```
    pub async fn fetch_bill(&self, session: &Session, start_date: impl AsRef<str>, end_date: impl AsRef<str>, page: u16, row: u16) -> CardResult<Vec<Bill>> {
        let res = card_request_handler(session, |client| {
            client.post(CARD_GET_BILL_URL)
                .form(&[
                    ("sdate", start_date.as_ref()),
                    ("edate", end_date.as_ref()),
                    ("account", self.id.as_ref()),
                    ("page", &page.to_string()),
                    ("row", &row.to_string()),
                ])
        }).await?;

        let json = res.json::<Map<String, Value>>().await?;

        if let Some(Value::Array(data)) = json.get("rows") {
            return Ok(
                data.iter()
                    .filter_map(Value::as_object)
                    .filter_map(Bill::from_json)
                    .collect()
            )
        }

        Err(Error::UnExceptedError {msg: "Website response format incorrect".to_string()})
    }
}
