//! 该模块提供成绩查询、绩点查询接口

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::errors::Error;
use crate::errors::mycqu::MyCQUResult;
use crate::mycqu::{Course, CQUSession};
use crate::mycqu::utils::{check_website_response, mycqu_request_handler};
use crate::session::Session;
use crate::utils::APIModel;
use crate::utils::consts::{MYCQU_API_GPA_RANKING_URL, MYCQU_API_SCORE_URL};

/// 成绩对象
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Score {
    /// 学期
    pub session: CQUSession,
    /// 课程
    pub course: Course,
    /// 成绩，可能为数字，也可能为字符（优、良等）
    score: Option<String>,
    /// 初修/重修
    study_nature: String,
    /// 必修/选修
    course_nature: String,
}

impl Score {
    /// 从json字典中解析[`Score`]
    fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        Some(
            Score {
                session: CQUSession::from_str(json_map.get("sessionName")?.as_str()?).ok()?,
                course: Course::from_json(json_map, None),
                score: json_map.get("effectiveScoreShow").and_then(Value::as_str).map(ToString::to_string),
                study_nature: json_map.get("studyNature")?.as_str()?.to_string(),
                course_nature: json_map.get("courseNature")?.as_str()?.to_string(),
            }
        )
    }

    /// 通过具有教务网权限的会话([`Session`])，获取成绩([`Vec<Score>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// use rsmycqu::mycqu::{access_mycqu, CQUSession, Score};
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_score() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = Score::fetch_self(&session, false);
    /// # }
    /// ```
    pub async fn fetch_self(session: &Session, is_minor: bool) -> MyCQUResult<Vec<Self>> {
        let res = mycqu_request_handler(session, |client| {
            client.get(MYCQU_API_SCORE_URL).query(&[("isMinorBoo", is_minor)])
        }).await?.json::<Map<String, Value>>().await?;

        check_website_response(&res)?;

        res.get("data").and_then(Value::as_object).map(|item|
            item.values().map_while(|obj|
                obj.get("stuScoreHomePgVoS").and_then(Value::as_array).map(|arr|
                    arr.iter().filter_map(Value::as_object).filter_map(Score::from_json).collect::<Vec<Score>>()
                )
            ).flatten().collect()
        ).ok_or(Error::UnExceptedError { msg: "Excepted field \"data\" is missing or not an object".to_string() })
    }
}

impl APIModel for Score {}

/// 绩点排名对象
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GPARanking {
    /// 学生总绩点
    pub gpa: f32,
    /// 专业排名
    pub major_ranking: Option<u16>,
    /// 年级排名
    pub grade_ranking: Option<u16>,
    /// 班级排名
    pub class_ranking: Option<u16>,
    /// 加权平均分
    pub weighted_avg: f32,
    /// 辅修加权平均分
    pub minor_weighted_avg: Option<f32>,
    /// 辅修绩点
    pub minor_gpa: Option<f32>,
}

impl GPARanking {
    /// 从json字典中解析[`GPARanking`]
    fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        Some(
            GPARanking {
                gpa: json_map.get("gpa").and_then(Value::as_str).and_then(|s| s.parse::<f32>().ok())?,
                major_ranking: json_map.get("majorRanking").and_then(Value::as_str).and_then(|s| s.parse::<u16>().ok()),
                grade_ranking: json_map.get("gradeRanking").and_then(Value::as_str).and_then(|s| s.parse::<u16>().ok()),
                class_ranking: json_map.get("classRanking").and_then(Value::as_str).and_then(|s| s.parse::<u16>().ok()),
                weighted_avg: json_map.get("weightedAvg").and_then(Value::as_str).and_then(|s| s.parse::<f32>().ok())?,
                minor_weighted_avg: json_map.get("minorWeightedAvg").and_then(Value::as_str).and_then(|s| s.parse::<f32>().ok()),
                minor_gpa: json_map.get("minorGpa").and_then(Value::as_str).and_then(|s| s.parse::<f32>().ok()),
            }
        )
    }

    /// 通过具有教务网权限的会话([`Session`])，获取绩点排名([`GPARanking`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// use rsmycqu::mycqu::{access_mycqu, CQUSession, GPARanking};
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_gpa_ranking() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = GPARanking::fetch_self(&session);
    /// # }
    /// ```
    pub async fn fetch_self(session: &Session) -> MyCQUResult<Self> {
        let res = mycqu_request_handler(session, |client| {
            client.get(MYCQU_API_GPA_RANKING_URL).query(&[("isMinorBoo", false)])
        }).await?.json::<Map<String, Value>>().await?;
        check_website_response(&res)?;

        res.get("data").and_then(Value::as_object).and_then(GPARanking::from_json)
            .ok_or(Error::UnExceptedError { msg: "Excepted field \"data\" is missing or not an array".to_string() })
    }
}

impl APIModel for GPARanking {}