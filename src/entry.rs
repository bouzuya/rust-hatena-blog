use std::str::FromStr;

use crate::EntryId;
use atom_syndication::Feed;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    author_name: String,
    categories: Vec<String>,
    pub content: String,
    draft: bool,
    id: EntryId,
    title: String,
    updated: String, // YYYY-MM-DDTHH:MM:SS
}

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

// FIXME
pub fn get_id(entry: &atom_syndication::Entry) -> Option<EntryId> {
    // https://blog.hatena.ne.jp/{HATENA_ID}/{BLOG_ID}/atom/entry/{ENTRY_ID}
    entry
        .links
        .iter()
        .find(|link| link.rel == "edit")
        .and_then(|link| link.href.split('/').last())
        .and_then(|id| id.parse().ok())
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("parse entry error")]
pub struct ParseEntry;

impl Entry {
    pub fn from_entry_xml(body: &str) -> Result<Entry, ParseEntry> {
        let xml = format!(
            "<feed>{}</feed>",
            body.strip_prefix(r#"<?xml version="1.0" encoding="utf-8"?>"#)
                .unwrap_or(body)
        );
        let feed = Feed::from_str(xml.as_str()).map_err(|_| ParseEntry)?;
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

    pub fn new(
        id: EntryId,
        title: String,
        author_name: String,
        categories: Vec<String>,
        content: String,
        updated: String,
        draft: bool,
    ) -> Self {
        Self {
            author_name,
            categories,
            content,
            draft,
            id,
            title,
            updated,
        }
    }

    pub fn to_json(&self) -> String {
        json!(self).to_string()
    }
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, str::FromStr};

    use anyhow::Context;
    use atom_syndication::{
        extension::{Extension, ExtensionMap},
        Category, Content, FixedDateTime, Link, Person, Text,
    };

    use super::*;

    fn new_dummy() -> anyhow::Result<Entry> {
        Ok(Entry::new(
            "ID".parse::<EntryId>()?,
            "TITLE".to_string(),
            "AUTHOR_NAME".to_string(),
            vec!["CATEGORY".to_string()],
            "CONTENT".to_string(),
            "2020-02-07T00:00:00Z".to_string(),
            true,
        ))
    }

    #[test]
    fn new() -> anyhow::Result<()> {
        assert_eq!(
            Entry::new(
                "ID1".parse::<EntryId>()?,
                "TITLE1".to_string(),
                "AUTHOR_NAME1".to_string(),
                vec!["CATEGORY1".to_string(), "CATEGORY2".to_string()],
                "CONTENT1".to_string(),
                "2020-02-07T23:59:59Z".to_string(),
                true,
            ),
            Entry {
                id: "ID1".parse::<EntryId>()?,
                title: "TITLE1".into(),
                author_name: "AUTHOR_NAME1".into(),
                categories: vec!["CATEGORY1".into(), "CATEGORY2".into()],
                content: "CONTENT1".into(),
                updated: "2020-02-07T23:59:59Z".into(),
                draft: true,
            }
        );
        Ok(())
    }

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
    fn from_entry_xml() -> anyhow::Result<()> {
        assert_eq!(
            Entry::from_entry_xml(GET_ENTRY_RESPONSE_XML),
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
        let xml = GET_ENTRY_RESPONSE_XML;
        let xml = format!(
            "<feed>{}</feed>",
            xml.strip_prefix(r#"<?xml version="1.0" encoding="utf-8"?>"#)
                .context("strip_prefix")?
        );
        let feed = atom_syndication::Feed::from_str(xml.as_str())?;
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

    #[test]
    fn to_json() -> anyhow::Result<()> {
        let entry = new_dummy()?;
        assert_eq!(
            entry.to_json(),
            r#"{"author_name":"AUTHOR_NAME","categories":["CATEGORY"],"content":"CONTENT","draft":true,"id":"ID","title":"TITLE","updated":"2020-02-07T00:00:00Z"}"#
        );
        Ok(())
    }
}
