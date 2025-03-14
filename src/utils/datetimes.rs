//! 该模块用于解析my.cqu.edu.cn返回的日期相关数据

use serde_with::serde_conv;

use crate::utils::models::Period;

/// 将短的日期字符解析为对应数字，“一”到“日”分别对应0～6，不匹配时返回None
fn parse_short_weekday(weekday: impl AsRef<str>) -> Option<u8> {
    match weekday.as_ref() {
        "一" => Some(0),
        "二" => Some(1),
        "三" => Some(2),
        "四" => Some(3),
        "五" => Some(4),
        "六" => Some(5),
        "日" => Some(6),
        _ => None,
    }
}

/// 将长的日期字符解析为对应数字，“星期一”到“星期日”分别对应0～6，不匹配时返回None
fn parse_long_weekday(weekday: impl AsRef<str>) -> Option<u8> {
    match weekday.as_ref() {
        "星期一" => Some(0),
        "星期二" => Some(1),
        "星期三" => Some(2),
        "星期四" => Some(3),
        "星期五" => Some(4),
        "星期六" => Some(5),
        "星期日" => Some(6),
        _ => None,
    }
}

#[inline]
pub(crate) fn parse_weekday(weekday: &(impl AsRef<str> + ?Sized)) -> Option<u8> {
    parse_short_weekday(weekday).or_else(|| parse_long_weekday(weekday))
}

impl Period {
    pub(crate) fn parse_period_str(s: impl AsRef<str>) -> Option<Self> {
        let period: Vec<u8> = s
            .as_ref()
            .split('-')
            .map_while(|item| item.parse().ok())
            .collect();

        match period.len() {
            1 => Some(Period {
                start: period[0],
                end: period[0],
            }),
            2 => Some(Period {
                start: period[0],
                end: period[1],
            }),
            _ => None,
        }
    }

    pub(crate) fn parse_week_str(s: impl AsRef<str>) -> Vec<Self> {
        s.as_ref()
            .split(',')
            .map_while(Self::parse_period_str)
            .collect()
    }
}

serde_conv!(
    pub(crate) WeekDayStrHelper,
    u8,
    |weekday: &u8| weekday.to_string(),
    |weekday_str: String| -> Result<u8, String> {
        parse_weekday(&weekday_str).ok_or_else(|| {
            format!(
                "Invalid weekday string, source: {}, expected: \\d+",
                weekday_str
            )
        })
    }
);

serde_conv!(
    pub(crate) PeriodStrHelper,
    Period,
    |period: &Period| format!("{}-{}", period.start, period.end),
    |period_str: String| -> Result<Period, String> {
        Period::parse_period_str(&period_str).ok_or_else(|| {
            format!(
                "Invalid period string, source: {}, expected:\\d+-\\d+",
                period_str
            )
        })
    }
);

serde_conv!(
    pub(crate) WeekStrHelper,
    Vec<Period>,
    |weeks: &Vec<Period>| weeks
        .iter()
        .map(|period| format!("{}-{}", period.start, period.end))
        .collect::<Vec<String>>()
        .join(","),
    |weeks_str: String| -> Result<Vec<Period>, String> {
        Ok(Period::parse_week_str(&weeks_str))
    }
);
