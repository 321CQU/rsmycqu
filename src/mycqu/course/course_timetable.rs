//! 课表对象，一个对象存储有相同课程、相同行课节次和相同星期的一批行课安排

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::errors::Error;
use crate::errors::mycqu::MyCQUResult;
use crate::mycqu::course::course::Course;
use crate::mycqu::course_day_time::CourseDayTime;
use crate::mycqu::utils::mycqu_request_handler;
use crate::session::Session;
use crate::utils::APIModel;
use crate::utils::consts::{MYCQU_API_ENROLL_TIMETABLE_URL, MYCQU_API_TIMETABLE_URL};
use crate::utils::models::Period;

/// 课表对象，一个对象存储有相同课程、相同行课节次和相同星期的一批行课安排
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CourseTimetable {
    /// 对应的课程
    pub course: Course,
    /// 学生数
    pub stu_num: Option<u16>,
    /// 行课地点，无则为[`None`]
    pub classroom: Option<String>,
    /// 行课周数
    pub weeks: Vec<Period>,
    /// 行课的星期和节次
    ///
    /// 若时间是整周（如真实地占用整周的军训和某些实习、虚拟地使用一周的思修实践）则为[`None`]
    pub day_time: Option<CourseDayTime>,
    /// 是否真实地占用整周（如军训和某些实习是真实地占用、思修实践是“虚拟地占用”）
    pub whole_week: bool,
    /// 行课教室名称
    pub classroom_name: Option<String>,
    /// 实验课各次实验内容
    pub expr_projects: Vec<String>,
}

impl CourseTimetable {
    /// 从json字典中解析[`CourseTimetable`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        let weeks: Vec<Period> =
            json_map.get("teachingWeekFormat").or_else(|| json_map.get("weeks"))
                .and_then(Value::as_str).map(Period::parse_week_str)?;

        Some(
            CourseTimetable {
                course: Course::from_json(json_map, None),
                stu_num: json_map.get("selectedStuNum").and_then(Value::as_str).and_then(|item| item.parse().ok()),
                classroom: json_map.get("position").and_then(Value::as_str).map(ToString::to_string),
                weeks,
                day_time: CourseDayTime::from_json(json_map),
                whole_week: json_map.get("wholeWeekOccupy").and_then(Value::as_bool).unwrap_or(false),
                classroom_name: json_map.get("roomName").and_then(Value::as_str).map(ToString::to_string),
                expr_projects: json_map.get("exprProjectName").and_then(Value::as_str).unwrap_or("")
                    .split(',').filter(|item| !item.is_empty()).map(ToString::to_string).collect(),
            }
        )
    }

    fn handle_json_response(res: &Map<String, Value>, target_field: impl AsRef<str>) -> MyCQUResult<Vec<Self>> {
        res.get(target_field.as_ref()).and_then(Value::as_array)
            .map(|result| result.iter().filter_map(Value::as_object).filter_map(CourseTimetable::from_json).collect())
            .ok_or(Error::UnExceptedError { msg: format!("Expected field \"{}\" is missing or format incorrect", target_field.as_ref()) })
    }

    /// 通过具有教务网权限的会话([`Session`])，获取当前学期课表([`Vec<CourseTimetable>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// use rsmycqu::mycqu::{access_mycqu, CourseTimetable, CQUSession};
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_curr_timetable() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = CourseTimetable::fetch_curr(&session, "2020xxxx", cqu_session.id.unwrap());
    /// # }
    /// ```
    pub async fn fetch_curr(session: &Session, student_id: impl AsRef<str>, cqu_session_id: u16) -> MyCQUResult<Vec<Self>> {
        let res = mycqu_request_handler(session, |client| {
            client.post(MYCQU_API_TIMETABLE_URL)
                .query(&[("sessionId", cqu_session_id)])
                .json(&vec![student_id.as_ref()])
        }).await?.json::<Map<String, Value>>().await?;

        Self::handle_json_response(&res, "classTimetableVOList")
    }

    /// 通过具有教务网权限的会话([`Session`])，获取用户已选课程的课表([`Vec<CourseTimetable>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// use rsmycqu::mycqu::{access_mycqu, CourseTimetable, CQUSession};
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_curr_timetable() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = CourseTimetable::fetch_enroll(&session, "2020xxxx");
    /// # }
    /// ```
    pub async fn fetch_enroll(session: &Session, student_id: impl AsRef<str>) -> MyCQUResult<Vec<Self>> {
        let res = mycqu_request_handler(session, |client| {
            client.get(format!("{}/{}", MYCQU_API_ENROLL_TIMETABLE_URL, student_id.as_ref()))
        }).await?.json::<Map<String, Value>>().await?;

        Self::handle_json_response(&res, "data")
    }
}


impl APIModel for CourseTimetable {}
