use serde_json::json;

use crate::mycqu::{
    course::{CQUSession, Course},
    exam::{Exam, Invigilator},
};

#[test]
fn test_parse_exam_from_string_number_api_response() {
    let json = json!({
        "week": "13",
        "weekDay": "4",
        "roomName": "D1337",
        "buildingName": "一教学楼-D区",
        "floorNum": "3",
        "courseName": "信号与系统（Ⅲ）",
        "courseCode": "EE21020",
        "batchId": "1901",
        "batchName": "非集中考试周",
        "studentId": "20240235",
        "seatNum": "5",
        "session": "2026春",
        "courseDeptShortName": "电气",
        "examDate": "2026-05-28",
        "examStuNum": "66",
        "startTime": "14:00",
        "endTime": "16:00",
        "simpleChiefinvigilatorVOS": [
            {
                "instDeptShortName": "电气",
                "instructor": "张莉"
            }
        ],
        "simpleAssistantInviVOS": [
            {
                "instDeptShortName": "输变电装备技术全国重点实验室",
                "instructor": "颜伟"
            }
        ]
    });

    let exam: Exam = serde_json::from_value(json).unwrap();

    assert_eq!(
        exam,
        Exam {
            course: Course {
                name: Some("信号与系统（Ⅲ）".to_string()),
                code: Some("EE21020".to_string()),
                course_num: None,
                dept: Some("电气".to_string()),
                credit: None,
                instructor: None,
                session: Some(CQUSession {
                    id: None,
                    year: 2026,
                    is_autumn: false,
                }),
            },
            batch: "非集中考试周".to_string(),
            batch_id: 1901,
            building: "一教学楼-D区".to_string(),
            floor: Some(3),
            room: "D1337".to_string(),
            stu_num: 66,
            date_str: "2026-05-28".to_string(),
            start_time_str: "14:00".to_string(),
            end_time_str: "16:00".to_string(),
            week: 13,
            weekday: 4,
            stu_id: "20240235".to_string(),
            seat_num: 5,
            chief_invigilator: vec![Invigilator {
                name: "张莉".to_string(),
                dept: "电气".to_string(),
            }],
            asst_invigilator: Some(vec![Invigilator {
                name: "颜伟".to_string(),
                dept: "输变电装备技术全国重点实验室".to_string(),
            }]),
        }
    );
}
