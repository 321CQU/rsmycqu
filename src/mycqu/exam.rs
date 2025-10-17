//! 考试查询

use serde_json::{Map, Value};

use crate::{
    errors::mycqu::MyCQUResult,
    mycqu::{
        course::Course,
        utils::{encrypt::encrypt_student_id, mycqu_request_handler},
    },
    session::{Client, Session},
    utils::{ApiModel, consts::MYCQU_API_EXAM_LIST_URL},
};

/// 监考员信息
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Invigilator {
    /// 监考员姓名
    #[serde(alias = "instructor")]
    pub name: String,
    /// 监考员所在学院（可能是简称，如"数统"）
    #[serde(alias = "instDeptShortName")]
    pub dept: String,
}

impl ApiModel for Invigilator {}

/// 考试信息
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    /// 考试对应的课程，其中学分 "credit"、教师 "instructor"、教学班号 "course_num" 可能无法获取（其值会设置为 `None`）
    pub course: Course,
    /// 考试批次，如 "非集中考试周"
    #[serde(alias = "batchName")]
    pub batch: String,
    /// 选课系统中考试批次的内部id
    #[serde(alias = "batchId")]
    pub batch_id: u16,
    /// 考场楼栋
    #[serde(alias = "buildingName")]
    pub building: String,
    /// 考场楼层
    #[serde(alias = "floorNum")]
    #[serde(default)]
    pub floor: Option<u16>,
    /// 考场地点
    #[serde(alias = "roomName")]
    pub room: String,
    /// 考场人数
    #[serde(alias = "examStuNum")]
    pub stu_num: u16,
    /// 考试日期字符串
    #[serde(alias = "examDate")]
    pub date_str: String,
    /// 考试开始时间
    #[serde(alias = "startTime")]
    pub start_time_str: String,
    /// 考试结束时间
    #[serde(alias = "endTime")]
    pub end_time_str: String,
    /// 周次
    pub week: u16,
    /// 星期，0为周一，6为周日
    #[serde(alias = "weekDay")]
    pub weekday: u8,
    /// 考生学号
    #[serde(alias = "studentId")]
    pub stu_id: String,
    /// 考生座号
    #[serde(alias = "seatNum")]
    pub seat_num: u16,
    /// 监考员
    #[serde(alias = "simpleChiefinvigilatorVOS")]
    #[serde(default)]
    pub chief_invigilator: Vec<Invigilator>,
    /// 副监考员
    #[serde(alias = "simpleAssistantInviVOS")]
    #[serde(default)]
    pub asst_invigilator: Option<Vec<Invigilator>>,
}

impl Exam {
    /// 通过具有教务网权限的会话([`Session`])，获取考表安排([`Vec<Exam>`])
    ///
    /// # Examples
    /// ```rust, no_run
    /// # use serde::de::Unexpected::Option;
    /// # use rsmycqu::mycqu::access_mycqu;
    /// # use rsmycqu::mycqu::course::CQUSession;
    /// # use rsmycqu::mycqu::exam::Exam;
    /// # use rsmycqu::session::Session;
    /// # use rsmycqu::sso::login;
    /// #
    /// # async fn fetch_exam_list() {
    /// let mut session = Session::new();
    /// let cqu_session = CQUSession {id: Some(1234), year: 2023, is_autumn: true};
    /// login(&mut session, "your_auth", "your_password", false).await.unwrap();
    /// access_mycqu(&mut session).await.unwrap();
    /// let user = Exam::fetch_all(&session, "your_student_id");
    /// # }
    /// ```
    pub async fn fetch_all(
        client: &Client,
        session: &Session,
        student_id: impl AsRef<str>,
    ) -> MyCQUResult<Vec<Exam>> {
        let mut res = mycqu_request_handler(client, session, |client| {
            client
                .get(MYCQU_API_EXAM_LIST_URL)
                .query(&[("studentId", encrypt_student_id(student_id))])
        })
        .await?
        .json::<Map<String, Value>>()
        .await?;

        Self::extract_array(&mut res, "data")
    }
}

impl ApiModel for Exam {}
