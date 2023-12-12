use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name, Predicate};
use crate::errors::page_parser::{PageParseError, PageParseResult};

#[cfg(test)]
mod tests;

#[inline]
fn get_node_by_id<'a, 'b>(
    document: &'a Document,
    node_name: &'a str,
    id: &'b str,
) -> PageParseResult<'b, Node<'a>> {
    // 寻找是`node_name`节点且id为`id`的元素
    document
        .find(Name(node_name).and(Attr("id", id)))
        .next() // 获取第一个满足上述标准的元素
        .ok_or(PageParseError::RequireInfoNotFound { target: id }) // 如果失败，返回`RequireInfoNotFound`
}

#[derive(Eq, PartialEq, Debug)]
pub struct SSOLoginPageData {
    pub login_croypto: String,
    pub login_page_flowkey: String,
}

pub(crate) fn sso_login_parser(
    login_html: impl AsRef<str>,
) -> PageParseResult<'static, SSOLoginPageData> {
    let document = Document::from(login_html.as_ref());

    Ok(SSOLoginPageData {
        login_croypto: get_node_by_id(&document, "p", "login-croypto")?.text(),
        login_page_flowkey: get_node_by_id(&document, "p", "login-page-flowkey")?.text(),
    })
}

#[cfg(feature = "card")]
pub(crate) fn card_access_parser(
    html: impl AsRef<str>,
) -> PageParseResult<'static, String> {
    let document = Document::from(html.as_ref());

    Ok(
        get_node_by_id(&document, "input", "ssoticketid")?
            .attr("value")
            .ok_or(PageParseError::RequireInfoNotFound { target: "ssoticketid" })?
            .to_string()
    )
}
