//! 校园课余额、账单查询接口

use reqwest::header::CONTENT_LENGTH;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_str};
use serde_with::serde_as;
use snafu::{OptionExt, ensure};

use crate::{
    card::utils::card_request_handler,
    errors,
    errors::{
        ApiError,
        card::{CardError, CardResult},
    },
    session::{Client, Session},
    utils::{
        ApiModel,
        consts::{CARD_GET_BILL_URL, CARD_GET_CARD_URL},
    },
};

/// 校园卡相关信息
#[serde_as]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Card {
    /// 校园卡id
    #[serde(alias = "acctNo")]
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, serde_with::DisplayFromStr)>")]
    pub id: u64,
    /// 账户余额，单位为分
    #[serde(alias = "acctAmt")]
    pub amount: u64,
}

impl ApiModel for Card {}

/// 校园卡账单相关信息
#[serde_as]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Bill {
    /// 交易名称
    #[serde(rename = "tranName")]
    pub name: String,
    /// 交易时间
    #[serde(alias = "tranDt")]
    pub date: String,
    /// 交易地点
    #[serde(alias = "mchAcctName")]
    pub place: String,
    /// 交易金额，单位为分
    #[serde(alias = "tranAmt")]
    pub tran_amount: i64,
    /// 账户余额，单位为分
    #[serde(alias = "acctAmt")]
    #[serde_as(deserialize_as = "serde_with::DisplayFromStr")]
    pub acc_amount: i64,
}

impl ApiModel for Bill {}

impl Card {
    /// 通过具有校园卡查询网址权限的会话([`Session`])，获取校园卡信息([`Card`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::card::{access_card, Card};
    /// # use rsmycqu::session::{Client, Session};
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_card() {
    /// # let client = Client::default();
    /// # let mut session = Session::new();
    /// login(&client, &mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_card(&client, &mut session).await.unwrap();
    /// let card = Card::fetch_self(&client, &mut session).await.unwrap();
    /// # }
    /// ```
    pub async fn fetch_self(client: &Client, session: &Session) -> CardResult<Card> {
        let res = card_request_handler(client, session, |client| {
            client.post(CARD_GET_CARD_URL).header(CONTENT_LENGTH, 0)
        })
        .await?;

        // the result is a json string, so parse response to string first
        let text = res.json::<String>().await?;
        let mut json = from_str::<Map<String, Value>>(&text).map_err(|_| ApiError::Website {
            msg: "Website response format incorrect".to_string(),
        })?;

        ensure!(
            json.get("respCode").and_then(Value::as_str) == Some("0000"),
            errors::WebsiteSnafu {
                msg: json
                    .get("respInfo")
                    .and_then(Value::as_str)
                    .unwrap_or("No Website Error")
                    .to_string(),
            }
        );

        json.get_mut("objs")
            .and_then(Value::as_array_mut)
            .and_then(|array| array.get_mut(0))
            .map(Value::take)
            .map(serde_json::from_value)
            .whatever_context::<&str, ApiError<CardError>>("Website response format incorrect")?
            .map_err(|err| ApiError::ModelParse {
                msg: format!("Deserialize error: {}", err),
            })
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
    /// # use rsmycqu::session::{Client, Session};
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_bill() {
    /// # let client = Client::default();
    /// # let mut session = Session::new();
    /// login(&client, &mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_card(&client, &mut session).await.unwrap();
    /// let card = Card::fetch_self(&client, &mut session).await.unwrap();
    /// let bills = card.fetch_bill(&client, &session, "2023-11-10", "2023-12-12", 1, 100).await.unwrap();
    /// # }
    /// ```
    pub async fn fetch_bill(
        &self,
        client: &Client,
        session: &Session,
        start_date: impl AsRef<str>,
        end_date: impl AsRef<str>,
        page: u16,
        row: u16,
    ) -> CardResult<Vec<Bill>> {
        let res = card_request_handler(client, session, |client| {
            client.post(CARD_GET_BILL_URL).form(&[
                ("sdate", start_date.as_ref()),
                ("edate", end_date.as_ref()),
                ("account", self.id.to_string().as_ref()),
                ("page", &page.to_string()),
                ("row", &row.to_string()),
            ])
        })
        .await?;

        let mut json = res.json::<Map<String, Value>>().await?;

        if let Some(Value::Array(data)) = json.get_mut("rows").map(Value::take) {
            data.into_iter()
                .map(serde_json::from_value)
                .collect::<Result<_, serde_json::Error>>()
                .map_err(|err| ApiError::ModelParse {
                    msg: format!("Deserialize error: {}", err),
                })
        } else {
            Err(ApiError::Website {
                msg: "Website response format incorrect".to_string(),
            })
        }
    }
}
