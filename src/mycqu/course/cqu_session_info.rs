//! 提供了某一学期的详细信息

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_with::{serde_as, serde_conv};

use super::CQUSession;
use crate::{
    errors::{mycqu::MyCQUResult, ApiError},
    mycqu::utils::mycqu_request_handler,
    session::Session,
    utils::{
        consts::{MYCQU_API_ALL_SESSION_INFO_URL, MYCQU_API_CURR_SESSION_INFO_URL},
        ApiModel,
    },
};

/// 重庆大学某一学期的详细信息
#[serde_as]
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct CQUSessionInfo {
    /// 对应的学期
    #[serde(flatten)]
    #[serde_as(
        deserialize_as = "serde_with::PickFirst<(_, CQUSessionHelper, serde_with::DisplayFromStr)>"
    )]
    pub session: CQUSession,
    /// 学期开始日期字符串（"yyyy-MM-dd"格式）
    #[serde(alias = "beginDate")]
    pub begin_date_str: Option<String>,
    /// 学期结束日期字符串（"yyyy-MM-dd"格式）
    #[serde(alias = "endDate")]
    pub end_date_str: Option<String>,
    /// 该学期是否为活跃学期（当前学期）
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, ActiveHelper)>")]
    pub active: bool,
}

serde_conv!(
    ActiveHelper,
    bool,
    |active: &bool| if *active { "Y" } else { "N" },
    |active: String| -> Result<bool, &'static str> {
        match active.as_str() {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err("Invalid active string"),
        }
    }
);

serde_conv!(
    CQUSessionHelper,
    CQUSession,
    |session: &CQUSession| {
        let season = if session.is_autumn { "秋" } else { "春" };
        let name = format!("{}{}", session.year, season);
        HashMap::<String, Value>::from_iter(
            vec![
                ("name".to_string(), name.into()),
                (
                    "id".to_string(),
                    session
                        .id
                        .map(|id| id.to_string().into())
                        .unwrap_or(Value::Null),
                ),
            ]
            .into_iter(),
        )
    },
    |session: HashMap<String, Value>| -> Result<CQUSession, &'static str> {
        let year = session
            .get("year")
            .and_then(|year| year.as_str().map(|year| year.parse::<u16>().ok()).flatten())
            .ok_or("year not found")?;
        let term = session
            .get("term")
            .and_then(|term| term.as_str().map(|term| term.chars().next()))
            .flatten();
        let id = session
            .get("id")
            .and_then(|id| id.as_str().map(|id| id.parse::<u16>().ok()).flatten());

        let session = CQUSession {
            id,
            year,
            is_autumn: term == Some('秋'),
        };
        Ok(CQUSession { id, ..session })
    }
);

impl CQUSessionInfo {
    /// 通过具有教务网权限的会话([`Session`])，从教务网获取全部包括了ID的详细学期信息([`CQUSessionInfo`])
    ///
    /// 返回的所有详细学期信息`begin_date_str`和`end_date_str`字段通常不为None
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSessionInfo;
    /// # use rsmycqu::sso::login;
    /// # use rsmycqu::session::Session;
    /// # async fn async_fetch_all_cqu_session_info() {
    /// # let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let cqu_session_infos = CQUSessionInfo::fetch_all(&session);
    /// # }
    /// ```
    pub async fn fetch_all(session: &Session) -> MyCQUResult<Vec<Self>> {
        let mut res =
            mycqu_request_handler(session, |client| client.get(MYCQU_API_ALL_SESSION_INFO_URL))
                .await?
                .json::<Map<String, Value>>()
                .await?;

        res.get_mut("sessionVOList")
            .and_then(Value::as_array_mut)
            .ok_or(ApiError::ModelParse {
                msg: "Expected field 'sessionVOList' is missing or not an array".into(),
            })
            .map(CQUSessionInfo::parse_json_array)?
    }

    /// 通过具有教务网权限的会话([`Session`])，从教务网获取包括了ID的当前学期详细信息([`CQUSessionInfo`])
    ///
    /// 返回的当前学期详细信息`begin_date_str`和`end_date_str`字段通常为None
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSessionInfo;
    /// # use rsmycqu::sso::login;
    /// # use rsmycqu::session::Session;
    /// # async fn async_fetch_all_cqu_session_info() {
    /// # let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let curr_cqu_session_info = CQUSessionInfo::fetch_curr(&session);
    /// # }
    /// ```
    pub async fn fetch_curr(session: &Session) -> MyCQUResult<Self> {
        let mut res = mycqu_request_handler(session, |client| {
            client.get(MYCQU_API_CURR_SESSION_INFO_URL)
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;
        res.get_mut("data")
            .map(Value::take)
            .ok_or(ApiError::ModelParse {
                msg: "Expected field \"data\" is missing or not an object".into(),
            })
            .map(serde_json::from_value)?
            .map_err(|e| ApiError::ModelParse {
                msg: format!("Model parse error: {e:?}"),
            })
    }
}

impl ApiModel for CQUSessionInfo {}
