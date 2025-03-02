//! 可选课程时间表信息

use std::{option::Option, vec::Vec};

use serde::{Deserialize, Serialize};
use serde_with::serde_conv;

use crate::{
    mycqu::course::CourseDayTime,
    utils::{ApiModel, datetimes::parse_weekday, models::Period},
};

/// 可选课程时间表信息
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnrollCourseTimetable {
    /// 上课周数，例如：["1-5", "7-9"]
    pub weeks: Vec<Period>,
    /// 上课时间，包含星期和课程时间段，例如：CourseDayTime { weekday: "星期二", period: "6-7" }
    pub time: Option<CourseDayTime>,
    /// 上课地点，例如："D1144"
    pub pos: Option<String>,
}

serde_conv!(
    pub(crate) EnrollCourseTimetableHelper,
    Vec<EnrollCourseTimetable>,
    |timetable: &Vec<EnrollCourseTimetable>| {
        let mut result = String::new();
        for (i, item) in timetable.iter().enumerate() {
            if i > 0 {
                result.push(';');
            }
            result.push_str(
                &item
                    .weeks
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
            result.push_str("周");
            if let Some(time) = &item.time {
                result.push_str(&format!(
                    " 星期{} {}-{}小节",
                    time.short_weekday(),
                    time.period.start,
                    time.period.end
                ));
            }
            if let Some(pos) = &item.pos {
                result.push_str(&format!(" &{}", pos));
            }
        }
        result
    },
    |timetable: &str| -> Result<_, std::convert::Infallible> {
        Ok(EnrollCourseTimetable::parse_timetable_str(timetable))
    }
);

impl EnrollCourseTimetable {
    /// 从字符串中生成具体待选课程上课时间信息
    ///
    /// 示例字符串"1-5,7-9周 星期二 6-7小节 &D1144 ;1-5,7-9周 星期五 3-4小节 &D1143 "
    ///
    /// ```rust
    /// # use rsmycqu::models::Period;
    /// # use rsmycqu::mycqu::course::CourseDayTime;
    /// # use rsmycqu::mycqu::enroll::EnrollCourseTimetable;
    /// #
    /// let timetable_str = "1-5,7-9周 星期二 6-7小节 &D1144 ;1-5,7-9周 星期五 3-4小节 &D1143 ";
    /// let timetable = EnrollCourseTimetable::parse_timetable_str(timetable_str);
    /// assert_eq!(timetable.len(), 2);
    /// assert_eq!(timetable[0], EnrollCourseTimetable {
    ///     weeks: vec![
    ///         Period { start: 1, end: 5 },
    ///         Period { start: 7, end: 9 }
    ///     ],
    ///     time: Some(CourseDayTime {
    ///         weekday: 1,
    ///         period: Period {
    ///             start: 6,
    ///             end: 7
    ///         }
    ///     }),
    ///     pos: Some("D1144".to_string())
    /// });
    /// assert_eq!(timetable[1], EnrollCourseTimetable {
    ///     weeks: vec![
    ///         Period { start: 1, end: 5 },
    ///         Period { start: 7, end: 9 }
    ///     ],
    ///     time: Some(CourseDayTime {
    ///         weekday: 4,
    ///         period: Period {
    ///             start: 3,
    ///             end: 4
    ///         }
    ///     }),
    ///     pos: Some("D1143".to_string())
    /// });
    /// ```
    pub fn parse_timetable_str(data: &str) -> Vec<EnrollCourseTimetable> {
        data.split(';')
            .map(|item| EnrollCourseTimetable {
                weeks: regex!(r"^(.*)周")
                    .captures(item)
                    .map(|mat| Period::parse_week_str(&mat[1]))
                    .unwrap_or(Vec::new()),
                time: regex!(r"星期(.) ([0-9])-([0-9])小节")
                    .captures(item)
                    .and_then(|mat| {
                        let weekday = parse_weekday(&mat[1]);
                        let (start, end) = (mat[2].parse(), mat[3].parse());
                        if let (Some(weekday), Ok(start), Ok(end)) = (weekday, start, end) {
                            Some(CourseDayTime {
                                weekday,
                                period: Period { start, end },
                            })
                        } else {
                            None
                        }
                    }),
                pos: regex!(r"&(.*)$")
                    .captures(item)
                    .map(|mat| mat[1].trim().to_string()),
            })
            .collect()
    }
}

impl ApiModel for EnrollCourseTimetable {}
