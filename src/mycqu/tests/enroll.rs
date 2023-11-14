use rstest::rstest;

use crate::errors::Error;
use crate::mycqu::enroll::{EnrollCourseInfo, EnrollCourseItem};
use crate::session::Session;
use crate::utils::test_fixture::access_mycqu_session;

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_enroll_course_info(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = EnrollCourseInfo::fetch_all(&session, true).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }
    let session = access_mycqu_session.await;
    EnrollCourseInfo::fetch_all(&session, true).await.unwrap();
    EnrollCourseInfo::fetch_all(&session, false).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_enroll_course_item(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = EnrollCourseItem::fetch_all(&session, "10000004872", true).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }
    let session = access_mycqu_session.await;
    EnrollCourseItem::fetch_all(&session, "10000004872", true)
        .await
        .unwrap();
    EnrollCourseItem::fetch_all(&session, "10000004872", false)
        .await
        .unwrap();
}
