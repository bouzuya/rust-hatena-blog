use crate::EntryId;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    pub author_name: String,
    pub categories: Vec<String>,
    pub content: String,
    pub draft: bool,
    pub edited: String,
    pub id: EntryId,
    pub published: String,
    pub title: String,
    pub updated: String,
}

impl Entry {
    pub fn new(
        id: EntryId,
        title: String,
        author_name: String,
        categories: Vec<String>,
        content: String,
        updated: String,
        published: String,
        edited: String,
        draft: bool,
    ) -> Self {
        Self {
            author_name,
            categories,
            content,
            draft,
            edited,
            id,
            published,
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
    use super::*;

    fn new_dummy() -> anyhow::Result<Entry> {
        Ok(Entry::new(
            "ID".parse::<EntryId>()?,
            "TITLE".to_string(),
            "AUTHOR_NAME".to_string(),
            vec!["CATEGORY".to_string()],
            "CONTENT".to_string(),
            "2020-02-07T00:00:00Z".to_string(),
            "2020-02-08T00:00:00Z".to_string(),
            "2020-02-09T00:00:00Z".to_string(),
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
                "2020-02-08T23:59:59Z".to_string(),
                "2020-02-09T23:59:59Z".to_string(),
                true,
            ),
            Entry {
                author_name: "AUTHOR_NAME1".into(),
                categories: vec!["CATEGORY1".into(), "CATEGORY2".into()],
                content: "CONTENT1".into(),
                draft: true,
                edited: "2020-02-09T23:59:59Z".to_string(),
                id: "ID1".parse::<EntryId>()?,
                published: "2020-02-08T23:59:59Z".to_string(),
                title: "TITLE1".into(),
                updated: "2020-02-07T23:59:59Z".into(),
            }
        );
        Ok(())
    }

    #[test]
    fn to_json() -> anyhow::Result<()> {
        let entry = new_dummy()?;
        assert_eq!(
            entry.to_json(),
            r#"{"author_name":"AUTHOR_NAME","categories":["CATEGORY"],"content":"CONTENT","draft":true,"edited":"2020-02-09T00:00:00Z","id":"ID","published":"2020-02-08T00:00:00Z","title":"TITLE","updated":"2020-02-07T00:00:00Z"}"#
        );
        Ok(())
    }
}
