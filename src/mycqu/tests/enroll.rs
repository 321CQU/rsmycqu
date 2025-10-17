use rstest::rstest;

use crate::{
    errors::ApiError,
    mycqu::enroll::{EnrollCourseInfo, EnrollCourseItem},
    session::Session,
    utils::test_fixture::{access_mycqu_session, shared_client},
};

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_enroll_course_info(
    #[future] access_mycqu_session: Session,
    shared_client: &'static crate::session::Client,
) {
    {
        let session = Session::new();
        let res = EnrollCourseInfo::fetch_all(shared_client, &session, true).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }
    let session = access_mycqu_session.await;
    EnrollCourseInfo::fetch_all(shared_client, &session, true)
        .await
        .unwrap();
    EnrollCourseInfo::fetch_all(shared_client, &session, false)
        .await
        .unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_enroll_course_item(
    #[future] access_mycqu_session: Session,
    shared_client: &'static crate::session::Client,
) {
    {
        let session = Session::new();
        let res = EnrollCourseItem::fetch_all(shared_client, &session, "10000004872", true).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }
    let session = access_mycqu_session.await;
    EnrollCourseItem::fetch_all(shared_client, &session, "10000004872", true)
        .await
        .unwrap();
    EnrollCourseItem::fetch_all(shared_client, &session, "10000004872", false)
        .await
        .unwrap();
}
