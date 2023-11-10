//! 提供了某一学期的详细信息

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::errors::Error;
use crate::errors::mycqu::MyCQUResult;
use crate::mycqu::CQUSession;
use crate::mycqu::utils::mycqu_request_handler;
use crate::session::Session;
use crate::utils::APIModel;
use crate::utils::consts::{MYCQU_API_ALL_SESSION_INFO_URL, MYCQU_API_CURR_SESSION_INFO_URL};

/// 重庆大学某一学期的详细信息
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct CQUSessionInfo {
    /// 对应的学期
    pub session: CQUSession,
    /// 学期开始日期字符串（"yyyy-MM-dd"格式）
    pub begin_date_str: Option<String>,
    /// 学期结束日期字符串（"yyyy-MM-dd"格式）
    pub end_date_str: Option<String>,
    /// 该学期是否为活跃学期（当前学期）
    pub active: bool,
}

impl CQUSessionInfo {
    /// 从json字典中解析[`CQUSessionInfo`]
    pub(crate) fn from_json(map: &Map<String, Value>) -> Option<Self> {
        if let (
            Some(Value::String(year)),
            Some(Value::String(term)),
            Some(Value::String(id))
        ) = (
            map.get("year"),
            map.get("term"),
            map.get("id")
        ) {
            if let (Some(id), Some(year)) = (
                id.parse::<u16>().ok(),
                year.parse::<u16>().ok()
            ) {
                let begin_date_str = map.get("beginDate")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
                let end_date_str = map.get("endDate")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);

                // `fetch_curr` 接口返回值不包括该项，为当前学期，所以默认值为`true`
                // `fetch_all` 接口返回值包括该项，`Y`表示活跃，`N`表示不活跃
                let active = map.get("active")
                    .and_then(Value::as_str)
                    .map_or(true, |str| str == "Y");

                return Some(
                    CQUSessionInfo {
                        session: CQUSession {
                            id: Some(id),
                            year,
                            is_autumn: term == "秋",
                        },
                        begin_date_str,
                        end_date_str,
                        active,
                    }
                );
            }
        }

        None
    }

    /// 通过具有教务网权限的会话([`Session`])，从教务网获取全部包括了ID的详细学期信息([`CQUSessionInfo`])
    ///
    /// 返回的所有详细学期信息`begin_date_str`和`end_date_str`字段通常不为None
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::sso::login;
    /// # use rsmycqu::mycqu::{access_mycqu, CQUSessionInfo};
    /// # use rsmycqu::session::Session;
    /// # async fn async_fetch_all_cqu_session_info() {
    /// # let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let cqu_session_infos = CQUSessionInfo::fetch_all(&session);
    /// # }
    /// ```
    pub async fn fetch_all(session: &Session) -> MyCQUResult<Vec<Self>> {
        let res = mycqu_request_handler(
            session, |client| client.get(MYCQU_API_ALL_SESSION_INFO_URL)).await?
            .json::<Map<String, Value>>().await?;

        res.get("sessionVOList").and_then(Value::as_array)
            .map(|all_session| all_session.iter().map_while(Value::as_object).map_while(CQUSessionInfo::from_json).collect())
            .ok_or(Error::UnExceptedError { msg: "Expected field 'sessionVOList' is missing or not an array".to_string() })
    }

    /// 通过具有教务网权限的会话([`Session`])，从教务网获取包括了ID的当前学期详细信息([`CQUSessionInfo`])
    ///
    /// 返回的当前学期详细信息`begin_date_str`和`end_date_str`字段通常为None
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::sso::login;
    /// # use rsmycqu::mycqu::{access_mycqu, CQUSessionInfo};
    /// # use rsmycqu::session::Session;
    /// # async fn async_fetch_all_cqu_session_info() {
    /// # let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let curr_cqu_session_info = CQUSessionInfo::fetch_curr(&session);
    /// # }
    /// ```
    pub async fn fetch_curr(session: &Session) -> MyCQUResult<Self> {
        let res = mycqu_request_handler(
            session, |client| client.get(MYCQU_API_CURR_SESSION_INFO_URL)).await?
            .json::<Map<String, Value>>().await?;
        res.get("data").and_then(Value::as_object).and_then(CQUSessionInfo::from_json)
            .ok_or(Error::UnExceptedError { msg: "Expected field \"data\" is missing or not an object".to_string() })
    }
}

impl APIModel for CQUSessionInfo {}
