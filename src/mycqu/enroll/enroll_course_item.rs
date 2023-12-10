//! 可选具体课程项

use std::option::Option;
use std::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::errors::mycqu::{MyCQUError, MyCQUResult};
use crate::errors::Error;
use crate::mycqu::course::Course;
use crate::mycqu::enroll::EnrollCourseTimetable;
use crate::mycqu::utils::mycqu_request_handler;
use crate::session::Session;
use crate::utils::consts::MYCQU_API_ENROLL_COURSE_DETAIL_URL;
use crate::utils::APIModel;

/// 可选具体课程，包含课程上课时间、上课教师、教室可容纳学生等信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnrollCourseItem {
    /// 可选具体课程id，每个可选具体课程具有唯一id，部分从属课程该值为`None`
    pub id: Option<String>,
    /// 可选具体课程所在学期ID，部分从属课程该值为`None`
    pub session_id: Option<String>,
    /// 是否已经选择该课程，部分从属课程该值为`None`
    pub checked: Option<bool>,
    /// 该具体课程所属课程ID，部分从属课程该值为`None`
    pub course_id: Option<String>,
    /// 课程信息
    pub course: Course,
    /// 具体课程类别，如：理论、实验
    pub course_type: String,
    /// 已选课程学生，部分从属课程该值为`None`
    pub selected_num: Option<u16>,
    /// 该课程容量上限，部分从属课程该值为`None`
    pub capacity: Option<u16>,
    /// 该课程从属课程列表，一般为理论课程的实验课
    pub children: Option<Vec<EnrollCourseItem>>,
    /// 所属校区，如D区，部分从属课程该值为`None`
    pub campus: Option<String>,
    /// 所从属具体课程id，如果不存在从属关系，该值为None
    pub parent_id: Option<String>,
    /// 课程时间表
    pub timetables: Vec<EnrollCourseTimetable>,
}

impl EnrollCourseItem {
    /// 从反序列化的可选具体课程字典中返回`EnrollCourseItem`
    pub fn from_json(data: &Map<String, Value>) -> Option<Self> {
        Some(Self {
            id: data.get("id").and_then(Value::as_str).map(String::from),
            session_id: data
                .get("sessionId")
                .and_then(Value::as_str)
                .map(String::from),
            checked: data.get("checked").and_then(Value::as_bool),
            course_id: data
                .get("courseId")
                .and_then(Value::as_str)
                .map(String::from),
            course: Course::from_json(data, None),
            course_type: data
                .get("classType")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            selected_num: data
                .get("selectedNum")
                .and_then(Value::as_u64)
                .map(u16::try_from)
                .transpose()
                .ok()?,
            capacity: data
                .get("stuCapacity")
                .and_then(Value::as_u64)
                .map(u16::try_from)
                .transpose()
                .ok()?,
            children: data
                .get("childrenList")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .map_while(Value::as_object)
                        .map_while(EnrollCourseItem::from_json)
                        .collect()
                }),
            campus: data
                .get("campusShortName")
                .and_then(Value::as_str)
                .map(String::from),
            parent_id: data
                .get("parentClassId")
                .and_then(Value::as_str)
                .map(String::from),
            timetables: data
                .get("classTime")
                .and_then(Value::as_str)
                .map(EnrollCourseTimetable::parse_timetable_str)
                .unwrap_or_default(),
        })
    }

    /// 通过具有教务网权限的会话([`Session`])，获取目标可选课程具体信息([`HashMap<String, Vec<Self>>`])
    ///
    /// `course_id` 为课程id，对应 [`EnrollCourseInfo`] 中的`id`属性
    /// `is_major` 为 `true` 时获取主修课程信息，为 `false` 时获取辅修课程信息
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// # use rsmycqu::mycqu::enroll::EnrollCourseItem;
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    ///
    /// # async fn fetch_enroll_course_item() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = EnrollCourseItem::fetch_all(&session, "target_course_id", true);
    /// # }
    /// ```
    pub async fn fetch_all(
        session: &Session,
        course_id: impl AsRef<str>,
        is_major: bool,
    ) -> MyCQUResult<Vec<Self>> {
        let res = mycqu_request_handler(session, |client| {
            client
                .get(format!(
                    "{}/{}",
                    MYCQU_API_ENROLL_COURSE_DETAIL_URL,
                    course_id.as_ref()
                ))
                .query(&[("selectionSource", if is_major { "主修" } else { "辅修" })])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;

        let select_course = res
            .get("selectCourseListVOs")
            .and_then(Value::as_array)
            .ok_or::<Error<MyCQUError>>(
                "Excepted field \"selectCourseListVOs\" is missing or not an array".into(),
            )?;

        if !select_course.is_empty() {
            select_course[0]
                .get("selectCourseVOList")
                .and_then(Value::as_array)
                .map(|array| {
                    array
                        .iter()
                        .map_while(Value::as_object)
                        .map_while(EnrollCourseItem::from_json)
                        .collect()
                })
                .ok_or("Excepted field \"selectCourseVOList\" is missing or not an array".into())
        } else {
            Ok(Vec::new())
        }
    }
}

impl APIModel for EnrollCourseItem {}
