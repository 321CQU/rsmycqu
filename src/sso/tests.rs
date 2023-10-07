use crate::session::Session;
use crate::sso::{encrypt::encrypt_password, login, LoginResult};
use crate::utils::test_fixture::{login_data, LoginData};
use rstest::*;

#[rstest]
fn test_login_page_encrypt() {
    let encrypted_password = encrypt_password("IGEOE4OMIBo=", "abc123456");

    assert_eq!(encrypted_password.unwrap(), "9p5YTOsEgya0j7w0dbg/CA==")
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_login(login_data: &LoginData) {
    let mut session = Session::new();
    let res = login(&mut session, &login_data.auth, &login_data.password, &false).await;

    assert_eq!(res.unwrap(), LoginResult::Success);
}
