use crate::mycqu::access_mycqu;
use crate::session::access_info::MY_CQU_ACCESS_INFO_KEY;
use crate::session::Session;
use crate::utils::test_tools::{login_session, runtime};
use rstest::*;
use tokio::runtime::Runtime;

#[rstest]
#[ignore]
fn test_access_mycqu(login_session: &Session, runtime: &Runtime) {
    let mut session = (*login_session).clone();
    runtime.block_on(access_mycqu(&mut session)).unwrap();
    assert!(session.access_info.contains_key(&MY_CQU_ACCESS_INFO_KEY));
}
