//! 可选课程信息

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    errors::{
        mycqu::{MyCQUError, MyCQUResult},
        ApiError,
    },
    mycqu::{
        course::Course,
        utils::{check_website_response, mycqu_request_handler},
    },
    session::Session,
    utils::{consts::MYCQU_API_ENROLL_COURSE_LIST_URL, ApiModel},
};

/// 可选课程信息
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnrollCourseInfo {
    /// 可选课程id
    pub id: String,
    /// 可选课程
    #[serde(flatten)]
    pub course: Course,
    /// 可选课程类型，如：公共基础课，主修专业课，非限制选修课等
    #[serde(alias = "courseCategory")]
    pub course_category: String,
    /// 课程类别，如：主修专业课，通识教育课程等
    #[serde(alias = "selectionArea")]
    pub course_type: String,
    /// 选课标识，如：已选，已选满等，当为 `None` 时标识无相关标记
    #[serde(alias = "courseEnrollSign")]
    #[serde(default)]
    pub enroll_sign: Option<String>,
    /// 课程属性，如必修，选修等
    #[serde(alias = "courseNature")]
    pub course_nature: String,
    /// 可选课程可选校区，如['D区'], ['A区', 'D区']等
    #[serde(alias = "campusShortNameSet")]
    #[serde(default)]
    pub campus: Vec<String>,
}

impl EnrollCourseInfo {
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
    /// let cqu_session = CQUSession { id: Some(1234), year: 2023, is_autumn: true };
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = EnrollCourseInfo::fetch_all(&session, true);
    /// # }
    /// ```
    pub async fn fetch_all(
        session: &Session,
        is_major: bool,
    ) -> MyCQUResult<HashMap<String, Vec<Self>>> {
        let mut res = mycqu_request_handler(session, |client| {
            client
                .get(MYCQU_API_ENROLL_COURSE_LIST_URL)
                .query(&[("selectionSource", if is_major { "主修" } else { "辅修" })])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;
        check_website_response(&res)?;

        let arrays =
            res.get_mut("data")
                .and_then(Value::as_array_mut)
                .ok_or(ApiError::ModelParse {
                    msg: "Excepted field \"data\" is missing or not an array".to_string(),
                })?;

        Ok(HashMap::from_iter(
            arrays
                .iter_mut()
                .filter_map(Value::as_object_mut)
                .filter_map(|item| {
                    let key = item
                        .get("selectionArea")
                        .and_then(Value::as_str)
                        .map(ToString::to_string);
                    let value = item.get_mut("courseVOList").and_then(Value::as_array_mut);
                    if let (Some(key), Some(value)) = (key, value) {
                        Some((
                            key,
                            EnrollCourseInfo::parse_json_array::<MyCQUError>(value).ok()?,
                        ))
                    } else {
                        None
                    }
                }),
        ))
    }
}

impl ApiModel for EnrollCourseInfo {}
