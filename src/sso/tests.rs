use rstest::*;

use crate::{
    session::Session,
    sso::{LoginResult, encrypt::encrypt_password, login},
    utils::test_fixture::{LoginData, login_data, shared_client},
};

#[rstest]
fn test_login_page_encrypt() {
    let encrypted_password = encrypt_password("IGEOE4OMIBo=", "abc123456");

    assert_eq!(encrypted_password.unwrap(), "9p5YTOsEgya0j7w0dbg/CA==")
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_login(login_data: &LoginData, shared_client: &'static crate::session::Client) {
    let mut session = Session::new();
    let res = login(
        shared_client,
        &mut session,
        &login_data.auth,
        &login_data.password,
        false,
    )
    .await;

    assert_eq!(res.unwrap(), LoginResult::Success);
}
