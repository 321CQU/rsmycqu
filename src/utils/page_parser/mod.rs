use scraper::{Html, Selector};

#[cfg(test)]
mod tests;

#[inline]
fn selector(selector: &str) -> Selector {
    Selector::parse(selector).expect("static selector should be valid")
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct SSOLoginPageData {
    pub(crate) login_croypto: String,
    pub(crate) login_page_flowkey: String,
}

pub(crate) fn sso_login_parser(login_html: impl AsRef<str>) -> Option<SSOLoginPageData> {
    let document = Html::parse_document(login_html.as_ref());

    Some(SSOLoginPageData {
        login_croypto: document
            .select(&selector("p#login-croypto"))
            .next()?
            .text()
            .collect(),
        login_page_flowkey: document
            .select(&selector("p#login-page-flowkey"))
            .next()?
            .text()
            .collect(),
    })
}

#[cfg(feature = "card")]
pub(crate) fn card_access_parser(html: impl AsRef<str>) -> Option<String> {
    let document = Html::parse_document(html.as_ref());

    document
        .select(&selector("input#ssoticketid"))
        .next()?
        .value()
        .attr("value")
        .map(ToString::to_string)
}
