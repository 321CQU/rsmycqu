use rstest::*;

use crate::errors::Error;
use crate::mycqu::{access_mycqu, Exam, GPARanking, Score, User};
use crate::session::Session;
use crate::utils::test_fixture::{access_mycqu_session, login_data, login_session, LoginData};

mod course;
#[rstest]
#[ignore]
#[tokio::test]
async fn test_access_mycqu(#[future] login_session: Session) {
    {
        let mut session = Session::new();
        let res = access_mycqu(&mut session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotLogin));
    }

    let mut session = login_session.await.clone();
    access_mycqu(&mut session).await.unwrap();
    assert!(session.mycqu_access_info.is_some());
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_user(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = User::fetch_self(&session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }
    User::fetch_self(&access_mycqu_session.await).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_score(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res1 = Score::fetch_self(&session, false).await;
        let res2 = Score::fetch_self(&session, true).await;
        assert!(res1.is_err());
        assert!(matches!(res1.unwrap_err(), Error::NotAccess));
        assert!(res2.is_err());
        assert!(matches!(res2.unwrap_err(), Error::NotAccess));
    }
    let session = access_mycqu_session.await;
    Score::fetch_self(&session, false).await.unwrap();
    Score::fetch_self(&session, true).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_gpa_ranking(#[future] access_mycqu_session: Session) {
    {
        let session = Session::new();
        let res = GPARanking::fetch_self(&session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }
    GPARanking::fetch_self(&access_mycqu_session.await).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_exam(#[future] access_mycqu_session: Session, login_data: &LoginData) {
    {
        let session = Session::new();
        let res = Exam::fetch_all(&session, &login_data.student_id).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }
    println!("{:?}", Exam::fetch_all(&access_mycqu_session.await, &login_data.student_id).await.unwrap());
}
