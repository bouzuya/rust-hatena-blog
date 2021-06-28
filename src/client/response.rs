use crate::client::PartialList;
use crate::{Entry, EntryId};
use anyhow::anyhow;
use atom_syndication::Feed;
use quick_xml::{
    events::{attributes::Attributes, Event},
    Reader,
};
use reqwest::Url;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

pub type CreateEntryResponse = MemberResponse;
pub type DeleteEntryResponse = EmptyResponse;
pub type GetEntryResponse = MemberResponse;
pub type ListCategoriesResponse = CategoryDocumentResponse;
pub type ListEntriesResponse = CollectionResponse;
pub type UpdateEntryResponse = MemberResponse;

#[derive(Debug, Eq, Error, PartialEq)]
#[error("parse entry error")]
pub struct ParseEntry;

#[derive(Debug, Eq, Error, PartialEq)]
#[error("parse category error")]
pub struct ParseCategory;

fn get_draft(entry: &atom_syndication::Entry) -> bool {
    entry
        .extensions
        .get("app")
        .and_then(|e| e.get("control"))
        .and_then(|children| children.iter().find(|e| &e.name == "app:control"))
        .and_then(|e| e.children.get("draft"))
        .and_then(|children| children.iter().find(|e| &e.name == "app:draft"))
        .and_then(|e| e.value.as_ref().map(|value| value == "yes"))
        .unwrap_or(false)
}

fn get_id(entry: &atom_syndication::Entry) -> Option<EntryId> {
    // https://blog.hatena.ne.jp/{HATENA_ID}/{BLOG_ID}/atom/entry/{ENTRY_ID}
    entry
        .links
        .iter()
        .find(|link| link.rel == "edit")
        .and_then(|link| link.href.split('/').last())
        .and_then(|id| id.parse().ok())
}

fn first_entry(feed: &Feed) -> Result<Entry, ParseEntry> {
    let entry = feed.entries().first().ok_or(ParseEntry)?;
    Ok(Entry::new(
        get_id(&entry).ok_or(ParseEntry)?,
        entry.title.to_string(),
        entry.authors.first().ok_or(ParseEntry)?.name.to_string(),
        entry
            .categories
            .iter()
            .map(|c| c.term.clone())
            .collect::<Vec<String>>(),
        entry
            .content
            .clone()
            .ok_or(ParseEntry)?
            .value
            .ok_or(ParseEntry)?,
        entry.updated.to_rfc3339(),
        get_draft(&entry),
    ))
}

fn from_entry_xml(body: &str) -> Result<Feed, ParseEntry> {
    let xml = format!(
        "<feed>{}</feed>",
        body.strip_prefix(r#"<?xml version="1.0" encoding="utf-8"?>"#)
            .unwrap_or(body)
    );
    Feed::from_str(xml.as_str()).map_err(|_| ParseEntry)
}

fn from_feed_xml(body: &str) -> Result<Feed, ParseEntry> {
    Feed::from_str(body).map_err(|_| ParseEntry)
}

fn categories_from_reader(
    ns_buf: &mut Vec<u8>,
    reader: &mut Reader<&[u8]>,
    _attrs: Attributes,
) -> anyhow::Result<Vec<String>> {
    let mut categories = vec![];
    let mut buf = vec![];
    loop {
        match reader.read_namespaced_event(&mut buf, ns_buf) {
            Ok(ns_event) => match ns_event {
                (Some(b"http://www.w3.org/2005/Atom"), Event::Empty(ref e))
                    if e.local_name() == b"category" =>
                {
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key == b"term" {
                            let value = attr.unescaped_value()?;
                            categories.push(String::from_utf8(value.to_vec())?);
                        }
                    }
                }
                (Some(b"http://www.w3.org/2007/app"), Event::End(ref e))
                    if e.local_name() == b"categories" =>
                {
                    break
                }
                (_, Event::Eof) => return Err(anyhow!("eof")),
                _ => {}
            },
            Err(e) => return Err(anyhow!(e)),
        }
        buf.clear();
    }
    Ok(categories)
}

fn from_category_document_xml(xml: &str) -> anyhow::Result<Vec<String>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut categories = None;
    let mut buf = vec![];
    let mut ns_buf = vec![];
    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
            Ok(ns_event) => match ns_event {
                (Some(b"http://www.w3.org/2007/app"), Event::Start(ref e))
                    if e.local_name() == b"categories" =>
                {
                    match categories {
                        None => {
                            categories = Some(categories_from_reader(
                                &mut ns_buf,
                                &mut reader,
                                e.attributes(),
                            )?);
                        }
                        Some(_) => {
                            return Err(anyhow!("too many <app:categories>"));
                        }
                    }
                }
                (Some(b"http://www.w3.org/2007/app"), Event::Empty(ref e))
                    if e.local_name() == b"categories" =>
                {
                    match categories {
                        None => {
                            return Err(anyhow!(
                                r#"<app:categories href="{CATEGORY_DOCUMENT}" /> is not supported"#
                            ));
                        }
                        Some(_) => {
                            return Err(anyhow!("too many <app:categories>"));
                        }
                    }
                }
                (_, Event::Eof) => break,
                _ => {}
            },
            Err(e) => return Err(anyhow!(e)),
        }
        buf.clear();
    }
    categories.ok_or_else(|| anyhow!("no <app:categories>"))
}

fn partial_list(feed: &Feed) -> Result<PartialList, ParseEntry> {
    Ok((
        feed.links
            .iter()
            .find(|link| link.rel == "next")
            .and_then(|link| Url::parse(link.href.as_str()).ok())
            .and_then(|href| {
                href.query_pairs()
                    .into_iter()
                    .find(|(name, _)| name == "page")
                    .map(|(_, value)| value.to_string())
            }),
        feed.entries
            .iter()
            .map(|entry| get_id(entry).ok_or(ParseEntry))
            .collect::<Result<Vec<EntryId>, ParseEntry>>()?,
    ))
}

#[derive(Debug, Eq, PartialEq)]
pub struct MemberResponse {
    body: String,
}

impl Display for MemberResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl From<String> for MemberResponse {
    fn from(body: String) -> Self {
        Self { body }
    }
}

impl From<MemberResponse> for String {
    fn from(response: MemberResponse) -> Self {
        response.body
    }
}

impl TryFrom<MemberResponse> for Entry {
    type Error = ParseEntry;

    fn try_from(response: MemberResponse) -> Result<Self, Self::Error> {
        let feed = from_entry_xml(response.body.as_str())?;
        first_entry(&feed)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct EmptyResponse;

impl Display for EmptyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl From<String> for EmptyResponse {
    fn from(_: String) -> Self {
        Self
    }
}

impl From<EmptyResponse> for String {
    fn from(_: EmptyResponse) -> Self {
        "".to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CategoryDocumentResponse {
    body: String,
}

impl Display for CategoryDocumentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl From<String> for CategoryDocumentResponse {
    fn from(body: String) -> Self {
        Self { body }
    }
}

impl From<CategoryDocumentResponse> for String {
    fn from(response: CategoryDocumentResponse) -> Self {
        response.body
    }
}

impl TryFrom<CategoryDocumentResponse> for Vec<String> {
    type Error = ParseEntry;

    fn try_from(response: CategoryDocumentResponse) -> Result<Self, Self::Error> {
        from_category_document_xml(response.body.as_str()).map_err(|_| ParseEntry)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CollectionResponse {
    body: String,
}

impl Display for CollectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl From<String> for CollectionResponse {
    fn from(body: String) -> Self {
        Self { body }
    }
}

impl From<CollectionResponse> for String {
    fn from(response: CollectionResponse) -> Self {
        response.body
    }
}

impl TryFrom<CollectionResponse> for PartialList {
    type Error = ParseEntry;

    fn try_from(response: CollectionResponse) -> Result<Self, Self::Error> {
        let feed = from_feed_xml(response.body.as_str())?;
        partial_list(&feed)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use atom_syndication::{
        extension::{Extension, ExtensionMap},
        Category, Content, FixedDateTime, Link, Person, Text,
    };

    use super::*;

    const GET_ENTRY_RESPONSE_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <id>tag:blog.hatena.ne.jp,2013:blog-{はてなID}-20000000000000-3000000000000000</id>
  <link rel="edit" href="https://blog.hatena.ne.jp/{はてなID}/{ブログID}/atom/edit/2500000000"/>
  <link rel="alternate" type="text/html" href="http://{ブログID}/entry/2013/09/02/112823"/>
  <author><name>{はてなID}</name></author>
  <title>記事タイトル</title>
  <updated>2013-09-02T11:28:23+09:00</updated>
  <published>2013-09-02T11:28:23+09:00</published>
  <app:edited>2013-09-02T11:28:23+09:00</app:edited>
  <summary type="text"> 記事本文 リスト1 リスト2 内容 </summary>
  <content type="text/x-hatena-syntax">
    ** 記事本文
    - リスト1
    - リスト2
    内容
  </content>
  <hatena:formatted-content type="text/html" xmlns:hatena="http://www.hatena.ne.jp/info/xmlns#">
    &lt;div class=&quot;section&quot;&gt;
    &lt;h4&gt;記事本文&lt;/h4&gt;

    &lt;ul&gt;
    &lt;li&gt;リスト1&lt;/li&gt;
    &lt;li&gt;リスト2&lt;/li&gt;
    &lt;/ul&gt;&lt;p&gt;内容&lt;/p&gt;
    &lt;/div&gt;
  </hatena:formatted-content>
  <category term="Scala" />
  <category term="Perl" />
  <app:control>
    <app:draft>no</app:draft>
  </app:control>
</entry>"#;

    #[test]
    fn from_entry_xml_test() -> anyhow::Result<()> {
        let feed = from_entry_xml(GET_ENTRY_RESPONSE_XML)?;
        assert_eq!(
            first_entry(&feed),
            Ok(Entry::new(
                "2500000000".parse::<EntryId>()?,
                "記事タイトル".to_string(),
                "{はてなID}".to_string(),
                vec!["Scala".to_string(), "Perl".to_string()],
                "\n    ** 記事本文\n    - リスト1\n    - リスト2\n    内容\n  ".to_string(),
                "2013-09-02T11:28:23+09:00".to_string(),
                false,
            ))
        );
        Ok(())
    }

    #[test]
    fn atom_syndication_parse_from_get_entry_xml() -> anyhow::Result<()> {
        let feed = from_entry_xml(GET_ENTRY_RESPONSE_XML)?;
        assert_eq!(feed.entries().len(), 1);
        let entry = feed.entries().first().unwrap().clone();
        assert_eq!(entry.title.as_str(), "記事タイトル");
        assert_eq!(
            entry.id,
            "tag:blog.hatena.ne.jp,2013:blog-{はてなID}-20000000000000-3000000000000000"
                .to_string()
        );
        assert_eq!(
            entry.updated,
            FixedDateTime::parse_from_rfc3339("2013-09-02T11:28:23+09:00")?
        );
        assert_eq!(
            entry.authors,
            vec![Person {
                name: "{はてなID}".to_string(),
                email: None,
                uri: None
            }]
        );
        assert_eq!(entry.contributors, vec![]);
        assert_eq!(
            entry.categories,
            vec![
                Category {
                    term: "Scala".to_string(),
                    scheme: None,
                    label: None,
                },
                Category {
                    term: "Perl".to_string(),
                    scheme: None,
                    label: None,
                }
            ]
        );
        assert_eq!(
            entry.links,
            vec![
                Link {
                    href: "https://blog.hatena.ne.jp/{はてなID}/{ブログID}/atom/edit/2500000000"
                        .to_string(),
                    rel: "edit".to_string(),
                    hreflang: None,
                    mime_type: None,
                    title: None,
                    length: None
                },
                Link {
                    href: "http://{ブログID}/entry/2013/09/02/112823".to_string(),
                    rel: "alternate".to_string(),
                    hreflang: None,
                    mime_type: Some("text/html".to_string()),
                    title: None,
                    length: None
                }
            ]
        );
        assert_eq!(
            entry.published,
            Some(FixedDateTime::parse_from_rfc3339(
                "2013-09-02T11:28:23+09:00"
            )?)
        );
        assert_eq!(entry.rights, None);
        assert_eq!(entry.source, None);
        assert_eq!(
            entry.summary,
            Some(Text::plain(" 記事本文 リスト1 リスト2 内容 ".to_string()))
        );
        assert_eq!(
            entry.content,
            Some(Content {
                base: None,
                lang: None,
                value: Some(
                    "\n    ** 記事本文\n    - リスト1\n    - リスト2\n    内容\n  ".to_string()
                ),
                src: None,
                content_type: Some("text/x-hatena-syntax".to_string()),
            })
        );
        assert_eq!(entry.extensions, {
            let mut extensions = ExtensionMap::new();
            extensions.insert("app".to_string(), {
                let mut app = BTreeMap::new();
                app.insert(
                    "control".to_string(),
                    vec![Extension {
                        name: "app:control".to_string(),
                        value: Some("".to_string()),
                        attrs: BTreeMap::new(),
                        children: {
                            let mut children = BTreeMap::new();
                            children.insert(
                                "draft".to_string(),
                                vec![Extension {
                                    name: "app:draft".to_string(),
                                    value: Some("no".to_string()),
                                    attrs: BTreeMap::new(),
                                    children: BTreeMap::new(),
                                }],
                            );
                            children
                        },
                    }],
                );
                app.insert(
                    "edited".to_string(),
                    vec![Extension {
                        name: "app:edited".to_string(),
                        value: Some("2013-09-02T11:28:23+09:00".to_string()),
                        attrs: BTreeMap::new(),
                        children: BTreeMap::new(),
                    }],
                );
                app
            });
            extensions.insert("hatena".to_string(), {
                let mut hatena = BTreeMap::new();
                hatena.insert(
                    "formatted-content".to_string(),
                    vec![Extension {
                        name: "hatena:formatted-content".to_string(),
                        value: Some("<div class=\"section\">\n    <h4>記事本文</h4>\n\n    <ul>\n    <li>リスト1</li>\n    <li>リスト2</li>\n    </ul><p>内容</p>\n    </div>".to_string()),
                        attrs: {
                          let mut attrs =  BTreeMap::new();
                          attrs.insert("type".to_string(), "text/html".to_string());
                          attrs.insert("xmlns:hatena".to_string(), "http://www.hatena.ne.jp/info/xmlns#".to_string());
                          attrs
                        },
                        children: BTreeMap::new(),
                    }],
                );
                hatena
            });
            extensions
        });
        Ok(())
    }

    const CATEGORY_DOCUMENT_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <app:categories
        xmlns:app="http://www.w3.org/2007/app"
        xmlns:atom="http://www.w3.org/2005/Atom"
        fixed="no">
      <atom:category term="Perl" />
      <atom:category term="Scala" />
    </app:categories>"#;

    #[test]
    fn category_document_test() -> anyhow::Result<()> {
        let categories = from_category_document_xml(CATEGORY_DOCUMENT_XML)?;
        assert_eq!(
            categories,
            ["Perl", "Scala"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
        Ok(())
    }
}
