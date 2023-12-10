//! 可选课程信息

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::errors::mycqu::MyCQUResult;
use crate::mycqu::course::Course;
use crate::mycqu::utils::{check_website_response, mycqu_request_handler};
use crate::session::Session;
use crate::utils::consts::MYCQU_API_ENROLL_COURSE_LIST_URL;
use crate::utils::APIModel;

/// 可选课程信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnrollCourseInfo {
    /// 可选课程id
    pub id: String,
    /// 可选课程
    pub course: Course,
    /// 可选课程类型，如：公共基础课，主修专业课，非限制选修课等
    pub course_category: String,
    /// 课程类别，如：主修专业课，通识教育课程等
    pub course_type: String,
    /// 选课标识，如：已选，已选满等，当为 `None` 时标识无相关标记
    pub enroll_sign: Option<String>,
    /// 课程属性，如必修，选修等
    pub course_nature: String,
    /// 可选课程可选校区，如['D区'], ['A区', 'D区']等
    pub campus: Vec<String>,
}

impl EnrollCourseInfo {
    /// 从json字典中解析[`EnrollCourseInfo`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        Some(EnrollCourseInfo {
            id: json_map.get("id")?.as_str()?.to_string(),
            course: Course {
                name: json_map
                    .get("name")
                    .and_then(Value::as_str)
                    .map(String::from),
                code: json_map
                    .get("codeR")
                    .and_then(Value::as_str)
                    .map(String::from),
                dept: json_map
                    .get("departmentName")
                    .and_then(Value::as_str)
                    .map(String::from),
                credit: json_map.get("credit").and_then(Value::as_f64),
                course_num: None,
                instructor: None,
                session: None,
            },
            course_category: json_map.get("courseCategory")?.as_str()?.to_string(),
            course_type: json_map.get("selectionArea")?.as_str()?.to_string(),
            enroll_sign: json_map
                .get("courseEnrollSign")
                .and_then(Value::as_str)
                .map(String::from),
            course_nature: json_map.get("courseNature")?.as_str()?.to_string(),
            campus: json_map
                .get("campusShortNameSet")
                .and_then(Value::as_array)
                .map(|array| {
                    array
                        .iter()
                        .map_while(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or(Vec::new()),
        })
    }

    /// 通过具有教务网权限的会话([`Session`])，获取选课信息([`HashMap<String, Vec<Self>>`])
    ///
    /// `is_major` 为 `true` 时获取主修课程信息，为 `false` 时获取辅修课程信息
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// use rsmycqu::mycqu::{access_mycqu, course::CQUSession};
    /// use rsmycqu::mycqu::enroll::EnrollCourseInfo;
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_enroll_course_info() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = EnrollCourseInfo::fetch_all(&session, true);
    /// # }
    /// ```
    pub async fn fetch_all(
        session: &Session,
        is_major: bool,
    ) -> MyCQUResult<HashMap<String, Vec<Self>>> {
        let res = mycqu_request_handler(session, |client| {
            client
                .get(MYCQU_API_ENROLL_COURSE_LIST_URL)
                .query(&[("selectionSource", if is_major { "主修" } else { "辅修" })])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;
        check_website_response(&res)?;

        res.get("data")
            .and_then(Value::as_array)
            .map(|array| {
                HashMap::from_iter(array.iter().map_while(Value::as_object).map_while(|item| {
                    let key = item
                        .get("selectionArea")
                        .and_then(Value::as_str)
                        .map(ToString::to_string);
                    let value = item.get("courseVOList").and_then(Value::as_array);
                    if let (Some(key), Some(value)) = (key, value) {
                        Some((
                            key,
                            value
                                .iter()
                                .map_while(Value::as_object)
                                .map_while(EnrollCourseInfo::from_json)
                                .collect::<Vec<EnrollCourseInfo>>(),
                        ))
                    } else {
                        None
                    }
                }))
            })
            .ok_or("Excepted field \"data\" is missing or not an array".into())
    }
}

impl APIModel for EnrollCourseInfo {}
