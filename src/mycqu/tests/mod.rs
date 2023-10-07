use crate::mycqu::access_mycqu;
use crate::session::access_info::MYCQU_ACCESS_INFO_KEY;
use crate::session::Session;
use crate::utils::test_fixture::login_session;
use rstest::*;

#[rstest]
#[ignore]
#[tokio::test]
async fn test_access_mycqu(#[future] login_session: Session) {
    let mut session = login_session.await.clone();
    access_mycqu(&mut session).await.unwrap();
    assert!(session.access_info.contains_key(&MYCQU_ACCESS_INFO_KEY));
}
