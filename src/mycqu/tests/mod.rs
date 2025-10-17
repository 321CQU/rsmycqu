use rstest::*;

use crate::{
    errors::ApiError,
    mycqu::{
        User, access_mycqu,
        exam::Exam,
        score::{GPARanking, Score},
    },
    session::Session,
    utils::test_fixture::{LoginData, access_mycqu_session, login_data, login_session},
};

mod course;
mod enroll;

#[rstest]
#[ignore]
#[tokio::test]
async fn test_access_mycqu(#[future] login_session: Session) {
    let client = crate::session::Client::default();
    {
        let mut session = Session::new();
        let res = access_mycqu(&client, &mut session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotLogin));
    }

    let mut session = login_session.await.clone();
    access_mycqu(&client, &mut session).await.unwrap();
    assert!(session.access_infos().mycqu_access_info.is_some());
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_user(#[future] access_mycqu_session: Session) {
    let client = crate::session::Client::default();
    {
        let session = Session::new();
        let res = User::fetch_self(&client, &session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }
    User::fetch_self(&client, &access_mycqu_session.await)
        .await
        .unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_score(#[future] access_mycqu_session: Session) {
    let client = crate::session::Client::default();
    {
        let session = Session::new();
        let res1 = Score::fetch_self(&client, &session, false).await;
        let res2 = Score::fetch_self(&client, &session, true).await;
        assert!(res1.is_err());
        assert!(matches!(res1.unwrap_err(), ApiError::NotAccess));
        assert!(res2.is_err());
        assert!(matches!(res2.unwrap_err(), ApiError::NotAccess));
    }
    let session = access_mycqu_session.await;
    println!(
        "{:?}",
        Score::fetch_self(&client, &session, false).await.unwrap()
    );
    Score::fetch_self(&client, &session, true).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_gpa_ranking(#[future] access_mycqu_session: Session) {
    let client = crate::session::Client::default();
    {
        let session = Session::new();
        let res = GPARanking::fetch_self(&client, &session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }
    GPARanking::fetch_self(&client, &access_mycqu_session.await)
        .await
        .unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_get_exam(#[future] access_mycqu_session: Session, login_data: &LoginData) {
    let client = crate::session::Client::default();
    {
        let session = Session::new();
        let res = Exam::fetch_all(&client, &session, &login_data.student_id).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }
    Exam::fetch_all(&client, &access_mycqu_session.await, &login_data.student_id)
        .await
        .unwrap();
}
