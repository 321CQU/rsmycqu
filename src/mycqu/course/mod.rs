//! 该模块提供课表相关信息查询接口

pub use course::*;
pub use course_day_time::*;
pub use course_timetable::*;
pub use cqu_session::*;
pub use cqu_session_info::*;

mod course;
mod course_day_time;
mod course_timetable;
mod cqu_session;
mod cqu_session_info;
