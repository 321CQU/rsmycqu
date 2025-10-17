//! 课表对象，一个对象存储有相同课程、相同行课节次和相同星期的一批行课安排

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_with::{StringWithSeparator, formats::CommaSeparator, serde_as};

use super::{Course, CourseDayTime};
use crate::{
    errors::mycqu::MyCQUResult,
    mycqu::utils::mycqu_request_handler,
    session::{Client, Session},
    utils::{
        ApiModel,
        consts::{MYCQU_API_ENROLL_TIMETABLE_URL, MYCQU_API_TIMETABLE_URL},
        datetimes::WeekStrHelper,
        models::Period,
    },
};

/// 课表对象，一个对象存储有相同课程、相同行课节次和相同星期的一批行课安排
#[serde_as]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CourseTimetable {
    /// 对应的课程
    #[serde(flatten)]
    pub course: Course,
    /// 学生数
    #[serde_as(deserialize_as = "Option<serde_with::DisplayFromStr>")]
    #[serde(alias = "selectedStuNum")]
    #[serde(default)]
    pub stu_num: Option<u16>,
    /// 行课地点，无则为[`None`]
    #[serde(alias = "position")]
    #[serde(default)]
    pub classroom: Option<String>,
    /// 行课周数
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, WeekStrHelper)>")]
    #[serde(alias = "teachingWeekFormat")]
    #[serde(alias = "weeks")]
    pub weeks: Vec<Period>,
    /// 行课的星期和节次
    ///
    /// 若时间是整周（如真实地占用整周的军训和某些实习、虚拟地使用一周的思修实践）则为[`None`]
    #[serde(flatten)]
    pub day_time: Option<CourseDayTime>,
    /// 是否真实地占用整周（如军训和某些实习是真实地占用、思修实践是“虚拟地占用”）
    #[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
    #[serde(alias = "wholeWeekOccupy")]
    #[serde(default)]
    pub whole_week: bool,
    /// 行课教室名称
    #[serde(alias = "roomName")]
    #[serde(default)]
    pub classroom_name: Option<String>,
    /// 实验课各次实验内容
    #[serde_as(deserialize_as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde(alias = "exprProjectName")]
    #[serde(default)]
    pub expr_projects: Vec<String>,
}

impl CourseTimetable {
    /// 通过具有教务网权限的会话([`Session`])，获取当前学期课表([`Vec<CourseTimetable>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::{CourseTimetable, CQUSession};
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    ///
    /// # async fn fetch_curr_timetable() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession { id: Some(1234), year: 2023, is_autumn: true };
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = CourseTimetable::fetch_curr(&client, &session, "2020xxxx", cqu_session.id.unwrap());
    /// # }
    /// ```
    pub async fn fetch_curr(
        client: &Client,
        session: &Session,
        student_id: impl AsRef<str>,
        cqu_session_id: u16,
    ) -> MyCQUResult<Vec<Self>> {
        let mut res = mycqu_request_handler(client, session, |client| {
            client
                .post(MYCQU_API_TIMETABLE_URL)
                .query(&[("sessionId", cqu_session_id)])
                .json(&vec![student_id.as_ref()])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;

        Self::extract_array(&mut res, "classTimetableVOList")
    }

    /// 通过具有教务网权限的会话([`Session`])，获取用户已选课程的课表([`Vec<CourseTimetable>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::{CourseTimetable, CQUSession};
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    ///
    /// # async fn fetch_curr_timetable() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession { id: Some(1234), year: 2023, is_autumn: true };
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = CourseTimetable::fetch_enroll(&client, &session, "2020xxxx");
    /// # }
    /// ```
    pub async fn fetch_enroll(
        client: &Client,
        session: &Session,
        student_id: impl AsRef<str>,
    ) -> MyCQUResult<Vec<Self>> {
        let mut res = mycqu_request_handler(client, session, |client| {
            client.get(format!(
                "{}/{}",
                MYCQU_API_ENROLL_TIMETABLE_URL,
                student_id.as_ref()
            ))
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;

        Self::extract_array(&mut res, "data")
    }
}

impl ApiModel for CourseTimetable {}
