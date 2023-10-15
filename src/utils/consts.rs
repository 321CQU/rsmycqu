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

#[cfg(feature = "sso")]
pub(crate) const SSO_ROOT_URL: &str = "https://sso.cqu.edu.cn";
#[cfg(feature = "sso")]
pub(crate) const SSO_LOGIN_URL: &str = formatcp!("{SSO_ROOT_URL}/login");
#[cfg(feature = "sso")]
pub(crate) const SSO_LOGOUT_URL: &str = formatcp!("{SSO_ROOT_URL}/logout");
