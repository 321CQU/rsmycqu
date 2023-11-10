//! 考试查询

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::errors::Error;
use crate::errors::mycqu::MyCQUResult;
use crate::mycqu::Course;
use crate::mycqu::utils::{encrypt::encrypt_student_id, mycqu_request_handler};
use crate::session::Session;
use crate::utils::APIModel;
use crate::utils::consts::MYCQU_API_EXAM_LIST_URL;
use crate::utils::datetimes::parse_weekday;

/// 监考员信息
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Invigilator {
    /// 监考员姓名
    pub name: String,
    /// 监考员所在学院（可能是简称，如"数统"）
    pub dept: String,
}

impl Invigilator {
    /// 从json字典中解析[`Invigilator`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        Some(
            Invigilator {
                name: json_map.get("instructor")?.as_str()?.to_string(),
                dept: json_map.get("instDeptShortName")?.as_str()?.to_string(),
            }
        )
    }
}

impl APIModel for Invigilator {}

/// 考试信息
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Exam {
    /// 考试对应的课程，其中学分 "credit"、教师 "instructor"、教学班号 "course_num" 可能无法获取（其值会设置为 `None`）
    pub course: Course,
    /// 考试批次，如 "非集中考试周"
    pub batch: String,
    /// 选课系统中考试批次的内部id
    pub batch_id: u16,
    /// 考场楼栋
    pub building: String,
    /// 考场楼层
    pub floor: Option<u16>,
    /// 考场地点
    pub room: String,
    /// 考场人数
    pub stu_num: u16,
    /// 考试日期字符串
    pub date_str: String,
    /// 考试开始时间
    pub start_time_str: String,
    /// 考试结束时间
    pub end_time_str: String,
    /// 周次
    pub week: u16,
    /// 星期，0为周一，6为周日
    pub weekday: u8,
    /// 考生学号
    pub stu_id: String,
    /// 考生座号
    pub seat_num: u16,
    /// 监考员
    pub chief_invigilator: Vec<Invigilator>,
    /// 副监考员
    pub asst_invigilator: Option<Vec<Invigilator>>,
}

impl Exam {
    /// 从json字典中解析[`Exam`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        Some(
            Exam {
                course: Course::from_json(json_map, None),
                batch: json_map.get("batchName")?.as_str()?.to_string(),
                batch_id: u16::try_from(json_map.get("batchId")?.as_u64()?).ok()?,
                building: json_map.get("buildingName")?.as_str()?.to_string(),
                floor: json_map.get("floorNum").and_then(Value::as_u64).map(u16::try_from).transpose().ok()?,
                room: json_map.get("roomName")?.as_str()?.to_string(),
                stu_num: u16::try_from(json_map.get("examStuNum")?.as_u64()?).ok()?,
                date_str: json_map.get("examDate")?.as_str()?.to_string(),
                start_time_str: json_map.get("startTime")?.as_str()?.to_string(),
                end_time_str: json_map.get("endTime")?.as_str()?.to_string(),
                week: u16::try_from(json_map.get("week")?.as_u64()?).ok()?,
                weekday: parse_weekday(&json_map.get("weekDay")?.as_str()?)?,
                stu_id: json_map.get("studentId")?.as_str()?.to_string(),
                seat_num: u16::try_from(json_map.get("seatNum")?.as_u64()?).ok()?,
                chief_invigilator: json_map.get("simpleChiefinvigilatorVOS")
                    .and_then(Value::as_array)
                    .map(|item| item.iter().map_while(Value::as_object).map_while(Invigilator::from_json).collect())
                    .unwrap_or_default(),
                asst_invigilator: json_map.get("simpleAssistantInviVOS")
                    .and_then(Value::as_array)
                    .map(|item| item.iter().map_while(Value::as_object).map_while(Invigilator::from_json).collect())
                    .filter(Vec::is_empty),
            }
        )
    }

    /// 通过具有教务网权限的会话([`Session`])，获取考表安排([`Vec<Exam>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// use rsmycqu::mycqu::{access_mycqu, CQUSession, Exam, GPARanking};
    /// use rsmycqu::session::Session;
    /// use rsmycqu::sso::login;
    ///
    /// # async fn fetch_exam_list() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = Exam::fetch_all(&session, "your_student_id");
    /// # }
    /// ```
    pub async fn fetch_all(session: &Session, student_id: impl AsRef<str>) -> MyCQUResult<Vec<Exam>> {
        let res = mycqu_request_handler(session, |client|
            client.get(MYCQU_API_EXAM_LIST_URL).query(&[("studentId", encrypt_student_id(student_id))]),
        ).await?.json::<Map<String, Value>>().await?;

        res.get("data").and_then(|item| item.get("content").and_then(Value::as_array))
            .map(|arr| arr.iter().filter_map(Value::as_object).filter_map(Exam::from_json).collect::<Vec<Exam>>())
            .ok_or(Error::UnExceptedError { msg: "Unexpected data format".to_string() })
    }
}

impl APIModel for Exam {}
