use crate::errors::page_parser::{PageParseError, PageParseResult};
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

#[derive(Eq, PartialEq, Debug)]
pub struct SSOLoginPageData {
    pub login_croypto: String,
    pub login_page_flowkey: String,
}

fn get_text_by_id<'a, 'b>(
    document: &'a Document,
    node_name: &'a str,
    id: &'b str,
) -> PageParseResult<'b, String> {
    match document.find(Name(node_name).and(Attr("id", id))).next() {
        Some(node) => Ok(node.text()),
        None => Err(PageParseError::RequireInfoNotFound { target: id }),
    }
}

pub(crate) fn sso_login_parser(login_html: &str) -> PageParseResult<SSOLoginPageData> {
    let document = Document::from(login_html);

    Ok(SSOLoginPageData {
        login_croypto: get_text_by_id(&document, "p", "login-croypto")?,
        login_page_flowkey: get_text_by_id(&document, "p", "login-page-flowkey")?,
    })
}
