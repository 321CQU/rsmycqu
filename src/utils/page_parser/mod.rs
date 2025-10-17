use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Name, Predicate},
};

#[cfg(test)]
mod tests;

#[inline]
fn get_node_by_id<'a>(document: &'a Document, node_name: &'a str, id: &str) -> Option<Node<'a>> {
    // 寻找是`node_name`节点且id为`id`的元素
    document.find(Name(node_name).and(Attr("id", id))).next()
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct SSOLoginPageData {
    pub(crate) login_croypto: String,
    pub(crate) login_page_flowkey: String,
}

pub(crate) fn sso_login_parser(login_html: impl AsRef<str>) -> Option<SSOLoginPageData> {
    let document = Document::from(login_html.as_ref());

    Some(SSOLoginPageData {
        login_croypto: get_node_by_id(&document, "p", "login-croypto")?.text(),
        login_page_flowkey: get_node_by_id(&document, "p", "login-page-flowkey")?.text(),
    })
}

#[cfg(feature = "card")]
pub(crate) fn card_access_parser(html: impl AsRef<str>) -> Option<String> {
    let document = Document::from(html.as_ref());

    get_node_by_id(&document, "input", "ssoticketid")?
        .attr("value")
        .map(ToString::to_string)
}
