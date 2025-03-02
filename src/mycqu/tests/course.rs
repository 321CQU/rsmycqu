use rstest::*;
use serde_json::{json, Value};

use crate::{
    errors::ApiError,
    mycqu::course::{CQUSession, CQUSessionInfo, Course, CourseDayTime, CourseTimetable},
    session::Session,
    utils::{
        models::Period,
        test_fixture::{access_mycqu_session, login_data, LoginData},
    },
};

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_all_session(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = CQUSession::fetch_all(&session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    let res = CQUSession::fetch_all(&access_mycqu_session.await)
        .await
        .unwrap();
    assert!(res.iter().all(|item| item.id.is_some()));
    assert!(!res.is_empty());
}

#[rstest]
fn test_parse_session_info_from_json() {
    let json1 = json!({"id": "1045", "year": "2023", "term": '秋', "beginDate": null, "endDate": null, "active": 'Y'});
    let session_info1: CQUSessionInfo = serde_json::from_value(json1).unwrap();
    assert_eq!(
        session_info1,
        CQUSessionInfo {
            active: true,
            begin_date_str: None,
            end_date_str: None,
            session: CQUSession {
                id: Some(1045),
                year: 2023,
                is_autumn: true
            },
        }
    );

    let json2 = json!({"id": "1046", "year": "2024", "term": '春', "beginDate": "2024-02-26", "endDate": "2024-08-25", "active": 'N'});
    let session_info2: CQUSessionInfo = serde_json::from_value(json2).unwrap();
    assert_eq!(
        session_info2,
        CQUSessionInfo {
            active: false,
            begin_date_str: Some("2024-02-26".to_string()),
            end_date_str: Some("2024-08-25".to_string()),
            session: CQUSession {
                id: Some(1046),
                year: 2024,
                is_autumn: false
            },
        }
    );
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_all_session_info(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = CQUSessionInfo::fetch_all(&session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    let res = CQUSessionInfo::fetch_all(&access_mycqu_session.await)
        .await
        .unwrap();
    assert!(!res.is_empty());
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_curr_session_info(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = CQUSessionInfo::fetch_curr(&session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    CQUSessionInfo::fetch_curr(&access_mycqu_session.await)
        .await
        .unwrap();
}

#[fixture]
fn example_course() -> Course {
    Course {
        code: Some("MT80007".to_string()),
        course_num: Some("000557-045".to_string()),
        credit: Some(0.0),
        dept: Some("马克思主义学院".to_string()),
        instructor: Some("李颖-30922[主讲];".to_string()),
        name: Some("形势与政策7".to_string()),
        session: None,
    }
}

#[rstest]
fn test_parse_course(example_course: Course) {
    let json_value: Value = serde_json::from_str(include_str!("course_timetable.json")).unwrap();
    let course: Course = serde_json::from_value(json_value).unwrap();

    assert_eq!(course, example_course);
}

#[rstest]
fn test_parse_course_day_time() {
    let json_value: Value = serde_json::from_str(include_str!("course_timetable.json")).unwrap();
    let course_day_time: CourseDayTime = serde_json::from_value(json_value).unwrap();

    assert_eq!(
        course_day_time,
        CourseDayTime {
            weekday: 4,
            period: Period { start: 3, end: 4 },
        }
    )
}

#[rstest]
fn test_parse_course_timetable(example_course: Course) {
    let json_value: Value = serde_json::from_str(include_str!("course_timetable.json")).unwrap();
    let course_timetable: CourseTimetable = serde_json::from_value(json_value).unwrap();

    assert_eq!(
        course_timetable,
        CourseTimetable {
            course: example_course,
            stu_num: Some(117),
            classroom: None,
            weeks: vec![Period { start: 14, end: 17 }],
            day_time: Some(CourseDayTime {
                weekday: 4,
                period: Period { start: 3, end: 4 },
            }),
            whole_week: false,
            classroom_name: Some("DYC101".to_string()),
            expr_projects: vec![],
        }
    )
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_curr_timetable(
    login_data: &LoginData,
    #[future] access_mycqu_session: Session,
) {
    {
        let session = Session::new();
        let res = CourseTimetable::fetch_curr(&session, &login_data.student_id, 0).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }
    let session = access_mycqu_session.await;
    let cqu_session = CQUSessionInfo::fetch_curr(&session).await.unwrap();

    println!(
        "{:?}",
        CourseTimetable::fetch_curr(
            &session,
            &login_data.student_id,
            cqu_session.session.id.unwrap(),
        )
        .await
        .unwrap()
    );
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_enroll_timetable(
    login_data: &LoginData,
    #[future] access_mycqu_session: Session,
) {
    {
        let session = Session::new();
        let res = CourseTimetable::fetch_enroll(&session, &login_data.student_id).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    CourseTimetable::fetch_enroll(&access_mycqu_session.await, &login_data.student_id)
        .await
        .unwrap();
}
