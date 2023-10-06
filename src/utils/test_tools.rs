use crate::mycqu::access_mycqu;
use crate::session::Session;
use crate::sso::{login, LoginResult};
use rstest::*;
use tokio::runtime::Runtime;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LoginData {
    pub auth: &'static str,
    pub password: &'static str,
}

#[fixture]
#[once]
pub(crate) fn login_data() -> LoginData {
    todo!("replace auth and password with your own");

    #[allow(unreachable_code)]
    LoginData {
        auth: "your_auth",
        password: "your_password",
    }
}

#[fixture]
#[once]
pub(crate) fn runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap()
}

#[fixture]
#[once]
pub(crate) fn login_session(runtime: &Runtime, login_data: &LoginData) -> Session {
    let mut session = Session::new();
    let res = runtime.block_on(login(
        &mut session,
        login_data.auth,
        login_data.password,
        &false,
    ));

    assert_eq!(res.unwrap(), LoginResult::Success);

    session
}

#[fixture]
#[once]
pub(crate) fn access_mycqu_session(login_session: &Session, runtime: &Runtime) -> Session {
    let mut session = (*login_session).clone();
    runtime.block_on(access_mycqu(&mut session)).unwrap();
    session
}
