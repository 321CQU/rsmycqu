//! 课程的星期和节次信息

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::utils::datetimes::parse_weekday;
use crate::utils::models::Period;
use crate::utils::APIModel;

/// 课程的星期和节次信息
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct CourseDayTime {
    /// 星期，0 为周一，6 为周日
    pub weekday: u8,
    /// 节次，第一个元素为开始节次，第二个元素为结束节次（该节次也包括在范围内）只有一节课时，两个元素相同
    pub period: Period,
}

impl CourseDayTime {
    /// 从json字典中解析[`CourseDayTime`]
    pub(crate) fn from_json(json_map: &Map<String, Value>) -> Option<Self> {
        if let (Some(Value::String(week_day_format)), Some(Value::String(period_format))) =
            (json_map.get("weekDayFormat"), json_map.get("periodFormat"))
        {
            if let (Some(weekday), Some(period)) = (
                parse_weekday(week_day_format),
                Period::parse_period_str(period_format),
            ) {
                return Some(CourseDayTime { weekday, period });
            }
        }

        None
    }
}

impl APIModel for CourseDayTime {}
