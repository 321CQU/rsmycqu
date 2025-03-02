//! 课程的星期和节次信息

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::utils::{ApiModel, datetimes::parse_weekday, models::Period};

/// 课程的星期和节次信息
#[serde_as]
#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct CourseDayTime {
    /// 星期，0 为周一，6 为周日
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, WeekDayStrHelper)>")]
    #[serde(alias = "weekDayFormat")]
    pub weekday: u8,
    /// 节次，第一个元素为开始节次，第二个元素为结束节次（该节次也包括在范围内）只有一节课时，两个元素相同
    #[serde_as(deserialize_as = "serde_with::PickFirst<(_, PeriodStrHelper)>")]
    #[serde(alias = "periodFormat")]
    pub period: Period,
}

impl<'de> Deserialize<'de> for CourseDayTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            WeekDay,
            WeekDayFormat,
            Period,
            PeriodFormat,
            #[serde(other)]
            Unknown,
        }

        struct CourseDayTimeVisitor;

        impl<'de> serde::de::Visitor<'de> for CourseDayTimeVisitor {
            type Value = CourseDayTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CourseDayTime")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CourseDayTime, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut weekday = None;
                let mut period = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::WeekDay => {
                            if weekday.is_none() {
                                weekday = map.next_value().ok();
                            }
                        }
                        Field::WeekDayFormat => {
                            if weekday.is_none() {
                                weekday =
                                    Some(parse_weekday(&map.next_value::<String>()?).ok_or_else(
                                        || serde::de::Error::custom("Invalid weekday"),
                                    )?);
                            }
                        }
                        Field::Period => {
                            if period.is_none() {
                                period = map.next_value().ok();
                            }
                        }
                        Field::PeriodFormat => {
                            if period.is_none() {
                                period = Some(
                                    Period::parse_period_str(&map.next_value::<String>()?)
                                        .ok_or_else(|| {
                                            serde::de::Error::custom("Invalid period")
                                        })?,
                                );
                            }
                        }
                        Field::Unknown => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let weekday = weekday.ok_or_else(|| serde::de::Error::missing_field("weekday"))?;
                let period = period.ok_or_else(|| serde::de::Error::missing_field("period"))?;

                Ok(CourseDayTime { weekday, period })
            }
        }

        const FIELDS: &[&str] = &["weekday", "weekDayFormat", "period", "periodFormat"];
        deserializer.deserialize_struct("CourseDayTime", FIELDS, CourseDayTimeVisitor)
    }
}

impl ApiModel for CourseDayTime {}

impl CourseDayTime {
    /// 获取星期的较短中文表示
    ///
    /// 例如：0 -> 一
    ///
    /// # Examples
    /// ```rust
    /// # use rsmycqu::models::Period;
    /// # use rsmycqu::mycqu::course::CourseDayTime;
    ///
    /// let course_day_time = CourseDayTime {
    ///     weekday: 0,
    ///     period: Period { start: 1, end: 2 },
    /// };
    ///
    /// assert_eq!(course_day_time.short_weekday(), "一");
    /// ```
    pub fn short_weekday(&self) -> &'static str {
        match self.weekday {
            0 => "一",
            1 => "二",
            2 => "三",
            3 => "四",
            4 => "五",
            5 => "六",
            6 => "日",
            _ => unreachable!(),
        }
    }

    /// 获取星期的较长中文表示
    ///
    /// 例如：0 -> 星期一
    ///
    /// # Examples
    /// ```rust
    /// # use rsmycqu::models::Period;
    /// # use rsmycqu::mycqu::course::CourseDayTime;
    ///
    /// let course_day_time = CourseDayTime {
    ///     weekday: 0,
    ///     period: Period { start: 1, end: 2 },
    /// };
    ///
    /// assert_eq!(course_day_time.long_weekday(), "星期一");
    pub fn long_weekday(&self) -> &'static str {
        match self.weekday {
            0 => "星期一",
            1 => "星期二",
            2 => "星期三",
            3 => "星期四",
            4 => "星期五",
            5 => "星期六",
            6 => "星期日",
            _ => unreachable!(),
        }
    }
}
