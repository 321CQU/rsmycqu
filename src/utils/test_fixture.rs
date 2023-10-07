use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;
use crate::mycqu::access_mycqu;
use crate::session::Session;
use crate::sso::{login, LoginResult};
use rstest::*;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LoginData {
    pub auth: String,
    pub password: String,
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

#[derive(Hash, Eq, PartialEq)]
pub(crate) enum SessionType {
    Login, AccessMycqu
}

pub(crate) struct ShareSessionMap(Mutex<HashMap<SessionType, Session>>);

impl Deref for ShareSessionMap {
    type Target = Mutex<HashMap<SessionType, Session>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ShareSessionMap {
    fn get_session(&self, session_type: &SessionType) -> Option<Session> {
        self.lock().ok()
            .and_then(|session_pool| {
            session_pool.get(session_type).cloned()
        })
    }
}

#[fixture]
#[once]
fn session_map() -> ShareSessionMap {
    ShareSessionMap(Mutex::new(HashMap::new()))
}

#[fixture]
pub(crate) async fn login_session(session_map: &ShareSessionMap, login_data: &LoginData) -> Session {
    match session_map.get_session(&SessionType::Login) {
        None => {
            let mut session = Session::new();
            let res = login(
                &mut session,
                &login_data.auth,
                &login_data.password,
                &false,
            ).await;

            assert_eq!(res.unwrap(), LoginResult::Success);

            {
                let mut session_pool = session_map.lock().unwrap();
                (*session_pool).insert(SessionType::Login, session.clone());
            }

            session
        }
        Some(session) => {
            session.clone()
        }
    }
}

#[fixture]
pub(crate) async fn access_mycqu_session(session_map: &ShareSessionMap, #[future] login_session: Session) -> Session {

    match session_map.get_session(&SessionType::AccessMycqu) {
        None => {
            let mut session = login_session.await;
            access_mycqu(&mut session).await.unwrap();

            {
                let mut session_pool = session_map.lock().unwrap();
                (*session_pool).insert(SessionType::AccessMycqu, session.clone());
            }

            session
        }
        Some(session) => {
            session.clone()
        }
    }
}