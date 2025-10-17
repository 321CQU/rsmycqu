use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Mutex, OnceLock},
};

use rstest::*;

#[cfg(feature = "card")]
use crate::card::access_card;
use crate::{
    mycqu::access_mycqu,
    session::{Client, Session},
    sso::{LoginResult, login},
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LoginData {
    pub auth: String,
    pub password: String,
    pub student_id: String,
}

#[fixture]
#[once]
pub(crate) fn login_data() -> LoginData {
    LoginData {
        auth: std::env::var("AUTH").unwrap(),
        password: std::env::var("PASSWORD").unwrap(),
        student_id: std::env::var("STUDENT_ID").unwrap(),
    }
}

#[derive(Hash, Eq, PartialEq)]
pub(crate) enum SessionType {
    Login,
    AccessMycqu,
    #[cfg(feature = "card")]
    AccessCard,
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
        self.lock()
            .ok()
            .and_then(|session_pool| session_pool.get(session_type).cloned())
    }
}

#[fixture]
#[once]
fn session_map() -> ShareSessionMap {
    ShareSessionMap(Mutex::new(HashMap::new()))
}

static SHARED_CLIENT: OnceLock<Client> = OnceLock::new();

#[fixture]
#[once]
pub(crate) fn shared_client() -> &'static Client {
    SHARED_CLIENT.get_or_init(|| {
        // 创建带连接池的client以在测试间共享TCP连接
        Client::custom(|builder| {
            builder
                .pool_max_idle_per_host(10)
                .pool_idle_timeout(std::time::Duration::from_secs(30))
                .tcp_keepalive(std::time::Duration::from_secs(30))
        })
        .unwrap_or_else(|_| Client::default())
    })
}

#[fixture]
pub(crate) async fn login_session(
    session_map: &ShareSessionMap,
    login_data: &LoginData,
    shared_client: &'static Client,
) -> Session {
    match session_map.get_session(&SessionType::Login) {
        None => {
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

            {
                let mut session_pool = session_map.lock().unwrap();
                (*session_pool).insert(SessionType::Login, session.clone());
            }

            session
        }
        Some(session) => session.clone(),
    }
}

#[fixture]
pub(crate) async fn access_mycqu_session(
    session_map: &ShareSessionMap,
    shared_client: &'static Client,
    #[future] login_session: Session,
) -> Session {
    match session_map.get_session(&SessionType::AccessMycqu) {
        None => {
            let mut session = login_session.await;
            access_mycqu(shared_client, &mut session).await.unwrap();

            {
                let mut session_pool = session_map.lock().unwrap();
                (*session_pool).insert(SessionType::AccessMycqu, session.clone());
            }

            session
        }
        Some(session) => session.clone(),
    }
}

#[cfg(feature = "card")]
#[fixture]
pub(crate) async fn access_card_session(
    session_map: &ShareSessionMap,
    shared_client: &'static Client,
    #[future] login_session: Session,
) -> Session {
    match session_map.get_session(&SessionType::AccessCard) {
        None => {
            let mut session = login_session.await;
            access_card(shared_client, &mut session).await.unwrap();

            {
                let mut session_pool = session_map.lock().unwrap();
                (*session_pool).insert(SessionType::AccessCard, session.clone());
            }

            session
        }
        Some(session) => session.clone(),
    }
}
