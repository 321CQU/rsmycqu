//! 学期相关信息接口

use std::{collections::HashMap, fmt::Display, future::Future, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, serde_conv};

use crate::{
    errors::{
        ApiError,
        mycqu::{MyCQUError, MyCQUResult},
    },
    mycqu::utils::mycqu_request_handler,
    session::{Client, Session},
    utils::{ApiModel, consts::MYCQU_API_SESSION_URL},
};

/// 重庆大学的某一学期
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct CQUSession {
    /// 学期ID
    pub id: Option<u16>,
    /// 行课年份
    pub year: u16,
    /// 是否为秋季学期
    pub is_autumn: bool,
}

serde_conv!(
    CQUSessionHelper,
    CQUSession,
    |session: &CQUSession| {
        let season = if session.is_autumn { "秋" } else { "春" };
        let name = format!("{}{}", session.year, season);
        HashMap::<String, Option<String>>::from_iter(
            vec![
                ("name".to_string(), Some(name.into())),
                ("id".to_string(), session.id.map(|id| id.to_string()).into()),
            ]
            .into_iter(),
        )
    },
    |session: HashMap::<String, Option<String>>| -> Result<CQUSession, &'static str> {
        let name = session.get("name").ok_or("name not found")?;
        let id = session
            .get("id")
            .and_then(|id| id.as_deref().map(|id| id.parse::<u16>().ok()).flatten());
        let session = name
            .as_ref()
            .map(|name| CQUSession::from_str(&name).map_err(|_| "CQUSession parse error"))
            .ok_or("name parse error")??;
        Ok(CQUSession { id, ..session })
    }
);

impl Display for CQUSession {
    /// 将[`CQUSession`]转为字符串形式
    ///
    /// # Example
    /// ```rust
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// let cqu_session = CQUSession { id: None, year: 2023, is_autumn: true };
    ///
    /// assert_eq!("2023秋", cqu_session.to_string())
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.year,
            if self.is_autumn { "秋" } else { "春" }
        )
    }
}

impl FromStr for CQUSession {
    type Err = ApiError<MyCQUError>;

    /// 通过正则表达式匹配字符串
    ///
    /// # Example
    /// ```rust
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// let cqu_session: CQUSession = "2023年秋".parse().unwrap();
    /// assert_eq!(cqu_session, CQUSession { id: None, year: 2023, is_autumn: true })
    /// ```
    ///
    /// ```rust
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// let cqu_session: CQUSession = "2023年春".parse().unwrap();
    /// assert_eq!(cqu_session, CQUSession { id: None, year: 2023, is_autumn: false })
    /// ```
    ///
    /// 以下调用方式会抛出[`ApiError<MyCQUError::CQUSessionParseError>::InnerError`]异常
    /// ```rust, should_panic
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// let cqu_session: CQUSession = "abced".parse().unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        regex!(r"^([0-9]{4})年?([春秋])$")
            .captures(s)
            .and_then(|captures| {
                if let (Some(year), Some(season)) = (captures.get(1), captures.get(2)) {
                    Some((year.as_str(), season.as_str()))
                } else {
                    None
                }
            })
            .map(|(year, season)| CQUSession {
                id: None,
                year: year.parse().unwrap(),
                is_autumn: season == "秋",
            })
            .ok_or(ApiError::ModelParse {
                msg: "CQUSession parse error".to_string(),
            })
    }
}

impl CQUSession {
    /// 当[`CQUSession.id`]为[`None`]时，调用`session_info_provider`尝试获取，如果成果则返回对应ID值并设置该对象，否则返回[`None`]
    /// 当[`CQUSession.id`]不为[`None`]时，返回[`CQUSession.id`]
    ///
    /// 如果您通过[`CQUSession.fetch_all`]获取学期信息，则所有学期的ID值会被正确设置
    /// 然而，通过字符串创建的学期没有ID信息，为此，我们提供了可选的`session_info_provider`
    /// 这允许你从外部提供一个函数/闭包来查询某一学期对应的ID，这允许你自由的决定在学期变量无ID时如何获取该ID的行为
    ///
    /// 下面是一个当无ID信息时，通过查询来获取ID的例子
    /// ```rust, no_run
    /// # use rsmycqu::session::Session;
    /// # use std::str::FromStr;
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// # async fn example() {
    /// async fn session_info_provider(year: u16, is_autumn: bool) -> Option<u16> {
    ///     let session = Session::new();
    ///     let mut all_session = CQUSession::fetch_all(&session).await.unwrap();
    ///     let target = all_session.iter().find(|item| item.year == year && item.is_autumn == is_autumn);
    ///     target.and_then(|item| item.id)
    /// }
    /// let mut cqu_session = CQUSession::from_str("2023秋").unwrap();
    /// let id = cqu_session.id_or(Some(session_info_provider));
    /// # }
    /// ```
    pub async fn id_or<T, U>(&mut self, session_info_provider: Option<T>) -> Option<u16>
    where
        T: Fn(u16, bool) -> U,
        U: Future<Output = Option<u16>>,
    {
        if self.id.is_none()
            && let Some(session_info_provider) = session_info_provider
        {
            self.id = session_info_provider(self.year, self.is_autumn).await;
        }

        self.id
    }

    /// 通过具有教务网权限的会话([`Session`])，从教务网获取全部包括了ID的学期信息([`CQUSession`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// # use rsmycqu::sso::login;
    /// # use rsmycqu::session::Session;
    /// # async fn async_fetch_all_cqu_session() {
    /// let mut session = Session::new();
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let cqu_sessions = CQUSession::fetch_all(&client, &session);
    /// # }
    /// ```
    pub async fn fetch_all(client: &Client, session: &Session) -> MyCQUResult<Vec<Self>> {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct LocalCQUSessionHelper(#[serde_as(deserialize_as = "CQUSessionHelper")] CQUSession);

        Ok(
            mycqu_request_handler(client, session, |client| client.get(MYCQU_API_SESSION_URL))
                .await?
                .json::<Vec<LocalCQUSessionHelper>>()
                .await?
                .into_iter()
                .map(|item| item.0)
                .filter(|item| item.id.is_some())
                .collect::<Vec<_>>(),
        )
    }
}

impl ApiModel for CQUSession {}
