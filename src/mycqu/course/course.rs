//! 与具体行课时间无关的课程信息

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::mycqu::CQUSession;
use crate::utils::APIModel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 与具体行课时间无关的课程信息
pub struct Course {
    /// 课程名称
    pub name: Option<String>,
    /// 课程代码
    pub code: Option<String>,
    /// 教学班号，在无法获取时（如考表[`exam::Exam`]中）设为 [`None`]
    pub course_num: Option<String>,
    /// 开课学院， 在无法获取时（如成绩[`score::Score`]中）设为[`None`]
    pub dept: Option<String>,
    /// 学分，无法获取到（如在考表[`exam::Exam`]中）则为[`None`]
    pub credit: Option<f64>,
    /// 教师
    pub instructor: Option<String>,
    /// 学期，无法获取时则为[`None`]
    pub session: Option<CQUSession>,
}

impl Course {
    /// 从json字典中解析[`Course`]
    ///
    /// `session`留空时尝试从`json_map`中获取
    pub(crate) fn from_json(json_map: &Map<String, Value>, session: Option<&CQUSession>) -> Self {
        let cqu_session: Option<CQUSession>;
        if let Some(session) = session {
            cqu_session = Some(session.clone());
        } else {
            cqu_session = json_map
                .get("session")
                .and_then(Value::as_str)
                .and_then(|item| CQUSession::from_str(item).ok());
        }

        let instructor: Option<String> = if let Some(Value::String(instructor_name)) =
            json_map.get("instructorName")
        {
            Some(instructor_name.to_string())
        } else if let Some(Value::String(instructor_names)) = json_map.get("instructorNames") {
            Some(instructor_names.to_string())
        } else if let Some(Value::Array(instructors)) = json_map.get("classTimetableInstrVOList") {
            Some(
                instructors
                    .iter()
                    .map_while(Value::as_str)
                    .collect::<Vec<&str>>()
                    .join(", "),
            )
        } else {
            None
        };

        Course {
            name: json_map
                .get("courseName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            code: json_map
                .get("courseCode")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            course_num: json_map
                .get("classNbr")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            dept: json_map
                .get("courseDepartmentName")
                .or(json_map.get("courseDeptShortName"))
                .and_then(Value::as_str)
                .map(ToString::to_string),
            credit: json_map
                .get("credit")
                .or(json_map.get("courseCredit"))
                .and_then(Value::as_str)
                .and_then(|item| item.parse().ok()),
            instructor,
            session: cqu_session,
        }
    }
}

impl APIModel for Course {}
