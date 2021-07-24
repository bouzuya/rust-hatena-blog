use crate::{EntryId, FixedDateTime};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    pub author_name: String,
    pub categories: Vec<String>,
    pub content: String,
    pub draft: bool,
    pub edit_url: String,
    pub edited: FixedDateTime,
    pub id: EntryId,
    pub published: FixedDateTime,
    pub title: String,
    pub updated: FixedDateTime,
    pub url: String,
}

impl Entry {
    pub fn to_json(&self) -> String {
        json!(self).to_string()
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    fn new_dummy() -> anyhow::Result<Entry> {
        Ok(Entry {
            author_name: "AUTHOR_NAME".to_string(),
            categories: vec!["CATEGORY".to_string()],
            content: "CONTENT".to_string(),
            draft: true,
            edit_url: "https://blog.hatena.ne.jp/{はてなID}/{ブログID}/atom/edit/2500000000"
                .to_string(),
            edited: FixedDateTime::from_str("2020-02-09T00:00:00Z")?,
            id: "ID".parse::<EntryId>()?,
            published: FixedDateTime::from_str("2020-02-08T00:00:00Z")?,
            title: "TITLE".to_string(),
            updated: FixedDateTime::from_str("2020-02-07T00:00:00Z")?,
            url: "http://{ブログID}/entry/2013/09/02/112823".to_string(),
        })
    }

    #[test]
    fn to_json() -> anyhow::Result<()> {
        let entry = new_dummy()?;
        assert_eq!(
            entry.to_json(),
            concat!(
                r#"{"author_name":"AUTHOR_NAME""#,
                r#","categories":["CATEGORY"]"#,
                r#","content":"CONTENT""#,
                r#","draft":true"#,
                r#","edit_url":"https://blog.hatena.ne.jp/{はてなID}/{ブログID}/atom/edit/2500000000""#,
                r#","edited":"2020-02-09T00:00:00Z""#,
                r#","id":"ID""#,
                r#","published":"2020-02-08T00:00:00Z""#,
                r#","title":"TITLE""#,
                r#","updated":"2020-02-07T00:00:00Z""#,
                r#","url":"http://{ブログID}/entry/2013/09/02/112823""#,
                r#"}"#
            )
        );
        Ok(())
    }
}
