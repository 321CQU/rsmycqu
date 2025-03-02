//! 与具体行课时间无关的课程信息

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::CQUSession;
use crate::utils::ApiModel;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 与具体行课时间无关的课程信息
pub struct Course {
    /// 课程名称
    #[serde(alias = "courseName")]
    #[serde(default)]
    pub name: Option<String>,
    /// 课程代码
    #[serde_as(deserialize_as = "serde_with::FromInto<CourseCodeField>")]
    #[serde(flatten)]
    pub code: Option<String>,
    /// 教学班号，在无法获取时（如考表[`exam::Exam`]中）设为 [`None`]
    #[serde(alias = "classNbr")]
    #[serde(default)]
    pub course_num: Option<String>,
    /// 开课学院， 在无法获取时（如成绩[`score::Score`]中）设为[`None`]
    #[serde_as(deserialize_as = "serde_with::FromInto<DepartmentField>")]
    #[serde(flatten)]
    pub dept: Option<String>,
    /// 学分，无法获取到（如在考表[`exam::Exam`]中）则为[`None`]
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    #[serde(alias = "courseCredit")]
    #[serde(default)]
    pub credit: Option<f64>,
    /// 教师
    #[serde_as(deserialize_as = "serde_with::FromInto<InstructorField>")]
    #[serde(flatten)]
    pub instructor: Option<String>,
    /// 学期，无法获取时则为[`None`]
    #[serde(default)]
    #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")]
    pub session: Option<CQUSession>,
}

impl ApiModel for Course {}

serde_fallback!(
    InstructorField,
    String,
    instructor,
    fallback = [
        instructorName,
        instructorNames,
        classTimetableInstrVOList
    ],
    apply = [
        #[serde_with::apply(
            _ => #[serde_as(deserialize_as = "Option<serde_with::PickFirst<(_, InstructorVec)>>")]
        )]
    ]
);

serde_fallback!(
    DepartmentField,
    String,
    dept,
    fallback = [departmentName, courseDepartmentName, courseDeptShortName]
);

serde_fallback!(
    CourseCodeField,
    String,
    code,
    fallback = [courseR, courseCode]
);

serde_with::serde_conv!(
    InstructorVec,
    String,
    |name: &String| [name.clone()],
    |value: Vec<String>| -> Result<_, std::convert::Infallible> { Ok(value.join(",")) }
);
