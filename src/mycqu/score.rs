//! 该模块提供成绩查询、绩点查询接口

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_with::serde_as;
use snafu::OptionExt;

use crate::{
    errors,
    errors::mycqu::MyCQUResult,
    mycqu::{
        course::{CQUSession, Course},
        utils::{check_website_response, mycqu_request_handler},
    },
    session::Session,
    utils::{
        consts::{MYCQU_API_GPA_RANKING_URL, MYCQU_API_SCORE_URL},
        ApiModel,
    },
};

/// 成绩对象
#[serde_as]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Score {
    /// 学期
    #[serde(alias = "sessionName")]
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, serde_with::DisplayFromStr)>")]
    pub session: CQUSession,
    /// 课程
    #[serde(flatten)]
    pub course: Course,
    /// 成绩，可能为数字，也可能为字符（优、良等）
    #[serde_as(deserialize_as = "serde_with::FromInto<ScoreField>")]
    #[serde(flatten)]
    score: Option<String>,
    /// 初修/重修
    #[serde(alias = "studyNature")]
    study_nature: String,
    /// 必修/选修
    #[serde(alias = "courseNature")]
    course_nature: String,
}

serde_fallback!(
    ScoreField,
    String,
    score,
    fallback = [effectiveScoreShow],
    apply = [
        #[serde_with::apply(
            _ => #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
        )]
    ]
);

impl Score {
    /// 通过具有教务网权限的会话([`Session`])，获取成绩([`Vec<Score>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// # use rsmycqu::mycqu::score::Score;
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_score() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession{ id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = Score::fetch_self(&session, false);
    /// # }
    /// ```
    pub async fn fetch_self(session: &Session, is_minor: bool) -> MyCQUResult<Vec<Self>> {
        let mut res = mycqu_request_handler(session, |client| {
            client
                .get(MYCQU_API_SCORE_URL)
                .query(&[("isMinorBoo", is_minor)])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;

        check_website_response(&res)?;

        res.get_mut("data")
            .and_then(Value::as_object_mut)
            .context(errors::ModelParseSnafu {
                msg: "Excepted field \"data\" is missing or not an object".to_string(),
            })?
            .values_mut()
            .map(|obj| {
                obj.get_mut("stuScoreHomePgVoS")
                    .and_then(Value::as_array_mut)
                    .context(errors::ModelParseSnafu {
                        msg: "Failed to parse score list".to_string(),
                    })
                    .and_then(ApiModel::parse_json_array)
            })
            .try_fold(Vec::<Self>::new(), |mut acc, item| {
                acc.extend(item?);
                Ok(acc)
            })
    }
}

impl ApiModel for Score {}

/// 绩点排名对象
#[serde_as]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GPARanking {
    /// 学生总绩点
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, serde_with::DisplayFromStr)>")]
    pub gpa: f32,
    /// 专业排名
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    #[serde(alias = "majorRanking")]
    pub major_ranking: Option<u16>,
    /// 年级排名
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    #[serde(alias = "gradeRanking")]
    pub grade_ranking: Option<u16>,
    /// 班级排名
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    #[serde(alias = "classRanking")]
    pub class_ranking: Option<u16>,
    /// 加权平均分
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, serde_with::DisplayFromStr)>")]
    #[serde(alias = "weightedAvg")]
    pub weighted_avg: f32,
    /// 辅修加权平均分
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    #[serde(alias = "minorWeightedAvg")]
    pub minor_weighted_avg: Option<f32>,
    /// 辅修绩点
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    #[serde(alias = "minorGpa")]
    pub minor_gpa: Option<f32>,
}

impl GPARanking {
    /// 通过具有教务网权限的会话([`Session`])，获取绩点排名([`GPARanking`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// # use rsmycqu::mycqu::score::GPARanking;
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_gpa_ranking() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession{ id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = GPARanking::fetch_self(&session);
    /// # }
    /// ```
    pub async fn fetch_self(session: &Session) -> MyCQUResult<Self> {
        let mut res = mycqu_request_handler(session, |client| {
            client
                .get(MYCQU_API_GPA_RANKING_URL)
                .query(&[("isMinorBoo", false)])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;
        check_website_response(&res)?;

        res.get_mut("data")
            .map(Value::take)
            .context(errors::ModelParseSnafu {
                msg: "Excepted field \"data\" is missing or not an object".to_string(),
            })
            .map(serde_json::from_value)?
            .map_err(|e| errors::ApiError::ModelParse {
                msg: format!("Failed to parse GPA ranking: {e}"),
            })
    }
}

impl ApiModel for GPARanking {}
