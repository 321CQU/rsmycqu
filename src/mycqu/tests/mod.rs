use crate::mycqu::{access_mycqu, User};
use crate::session::Session;
use crate::utils::test_fixture::{access_mycqu_session, login_session};
use rstest::*;
use crate::errors::Error;

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
