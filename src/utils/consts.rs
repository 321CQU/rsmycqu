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
pub(crate) const MYCQU_SERVICE_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/authserver/authentication/cas");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_SESSION_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/timetable/optionFinder/session?blankOption=false");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_USER_URL: &str = formatcp!("{MYCQU_ROOT_URL}/authserver/simple-user");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_ALL_SESSION_INFO_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/resourceapi/session/list");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_CURR_SESSION_INFO_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/resourceapi/session/cur-active-session");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_TIMETABLE_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/timetable/class/timetable/student/my-table-detail");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_ENROLL_TIMETABLE_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/enrollment/timetable/student");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_SCORE_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/sam/score/student/score");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_GPA_RANKING_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/sam/score/student/studentGpaRanking");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_EXAM_LIST_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/exam/examTask/get-student-exam-tab-list");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_ENROLL_COURSE_LIST_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/enrollment/enrollment/course-list");

#[cfg(feature = "mycqu")]
pub(crate) const MYCQU_API_ENROLL_COURSE_DETAIL_URL: &str =
    formatcp!("{MYCQU_ROOT_URL}/api/enrollment/enrollment/courseDetails");

#[cfg(feature = "sso")]
pub(crate) const SSO_ROOT_URL: &str = "https://sso.cqu.edu.cn";

#[cfg(feature = "sso")]
pub(crate) const SSO_LOGIN_URL: &str = formatcp!("{SSO_ROOT_URL}/login");

#[cfg(feature = "sso")]
pub(crate) const SSO_LOGOUT_URL: &str = formatcp!("{SSO_ROOT_URL}/logout");

#[cfg(feature = "card")]
pub(crate) const CARD_SERVICE_URL: &str = "http://card.cqu.edu.cn:7280/ias/prelogin?sysid=FWDT";

#[cfg(feature = "card")]
pub(crate) const CARD_HALL_TICKET_URL: &str = "http://card.cqu.edu.cn/cassyno/index";

#[cfg(feature = "card")]
pub(crate) const CARD_PAGE_URL: &str = "http://card.cqu.edu.cn/Page/Page";

#[cfg(feature = "card")]
pub(crate) const CARD_PAGE_TICKET_POST_FORM_URL: &str =
    "http://card.cqu.edu.cn:8080/blade-auth/token/thirdToToken/fwdt";

#[cfg(feature = "card")]
pub(crate) const CARD_BLADE_AUTH_URL: &str = "http://card.cqu.edu.cn:8080/blade-auth/token/fwdt";

#[cfg(feature = "card")]
pub(crate) const CARD_GET_DORM_FEE_URL: &str =
    "http://card.cqu.edu.cn:8080/charge/feeitem/getThirdData";

#[cfg(feature = "card")]
pub(crate) const CARD_GET_CARD_URL: &str = "http://card.cqu.edu.cn/NcAccType/GetCurrentAccountList";

#[cfg(feature = "card")]
pub(crate) const CARD_GET_BILL_URL: &str = "http://card.cqu.edu.cn/NcReport/GetMyBill";

#[cfg(feature = "library")]
pub(crate) const LIB_ROOT_URL: &str = "http://lib.cqu.edu.cn";

#[cfg(feature = "library")]
pub(crate) const LIB_ACCESS_URL: &str = "http://lib.cqu.edu.cn:8002/api/Auth/AccessToken";
