use const_format::formatcp;

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_ROOT_URL: &str = "https://my.cqu.edu.cn";
#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_TOKEN_INDEX_URL: &str = formatcp!("{MYCQU_ROOT_URL}/enroll/token-index");
#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_TOKEN_URL: &str = formatcp!("{MYCQU_ROOT_URL}/authserver/oauth/token");
#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_AUTHORIZE_URL: &str = formatcp!("{MYCQU_ROOT_URL}/authserver/oauth/authorize?client_id=enroll-prod&response_type=code&scope=all&state=&redirect_uri={MYCQU_TOKEN_INDEX_URL}");
#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_SERVICE_URL: &str = formatcp!("{MYCQU_ROOT_URL}/authserver/authentication/cas");


#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_SESSION_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/timetable/optionFinder/session?blankOption=false");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_USER_URL: &str = formatcp!("{MYCQU_ROOT_URL}/authserver/simple-user");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_ALL_SESSION_INFO_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/resourceapi/session/list");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_CURR_SESSION_INFO_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/resourceapi/session/cur-active-session");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_TIMETABLE_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/timetable/class/timetable/student/table-detail");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_ENROLL_TIMETABLE_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/enrollment/timetable/student");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_SCORE_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/sam/score/student/score");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_GPA_RANKING_URL: &str = formatcp!("{MYCQU_ROOT_URL}/api/sam/score/student/studentGpaRanking");


#[cfg(feature = "sso")]
pub(crate) const SSO_ROOT_URL: &str = "https://sso.cqu.edu.cn";
#[cfg(feature = "sso")]
pub(crate) const SSO_LOGIN_URL: &str = formatcp!("{SSO_ROOT_URL}/login");
#[cfg(feature = "sso")]
pub(crate) const SSO_LOGOUT_URL: &str = formatcp!("{SSO_ROOT_URL}/logout");
