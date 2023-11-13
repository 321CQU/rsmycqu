use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

use crate::errors::page_parser::{PageParseError, PageParseResult};

#[derive(Eq, PartialEq, Debug)]
pub struct SSOLoginPageData {
    pub login_croypto: String,
    pub login_page_flowkey: String,
}

#[inline]
fn get_text_by_id<'a, 'b>(
    document: &'a Document,
    node_name: &'a str,
    id: &'b str,
) -> PageParseResult<'b, String> {
    // 寻找是`node_name`节点且id为`id`的元素
    document
        .find(Name(node_name).and(Attr("id", id)))
        .next()
        .map(|node| node.text()) // 获取第一个满足上述标准的元素并取其文本
        .ok_or(PageParseError::RequireInfoNotFound { target: id }) // 如果失败，返回`RequireInfoNotFound`
}

pub(crate) fn sso_login_parser(
    login_html: impl AsRef<str>,
) -> PageParseResult<'static, SSOLoginPageData> {
    let document = Document::from(login_html.as_ref());

    Ok(SSOLoginPageData {
        login_croypto: get_text_by_id(&document, "p", "login-croypto")?,
        login_page_flowkey: get_text_by_id(&document, "p", "login-page-flowkey")?,
    })
}
