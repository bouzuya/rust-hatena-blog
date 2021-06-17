use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    categories: Vec<String>,
    content: String,
    draft: bool,
    name: String, // author.name
    title: String,
    updated: String, // YYYY-MM-DDTHH:MM:SS
}

impl Entry {
    #[allow(dead_code)]
    pub fn new_dummy() -> Self {
        Self::new(
            "TITLE".to_string(),
            "NAME".to_string(),
            vec!["CATEGORY".to_string()],
            "CONTENT".to_string(),
            "2020-02-07T00:00:00Z".to_string(),
            true,
        )
    }

    pub fn new(
        title: String,
        name: String,
        categories: Vec<String>,
        content: String,
        updated: String,
        draft: bool,
    ) -> Self {
        Self {
            categories,
            content,
            draft,
            name,
            title,
            updated,
        }
    }

    pub fn to_xml(&self) -> String {
        let registry = Handlebars::new();
        registry
            .render_template(
                r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <title>{{title}}</title>
  <author><name>{{name}}</name></author>
  <content type="text/plain">{{content}}</content>
  <updated>{{updated}}</updated>
  {{#each categories}}<category term="{{this}}" />{{/each}}
  <app:control>
    <app:draft>{{#if draft}}yes{{else}}no{{/if}}</app:draft>
  </app:control>
</entry>"#,
                &json!(self),
            )
            .expect("render_template")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Entry::new(
                "TITLE1".to_string(),
                "NAME1".to_string(),
                vec!["CATEGORY1".to_string(), "CATEGORY2".to_string()],
                "CONTENT1".to_string(),
                "2020-02-07T23:59:59Z".to_string(),
                true,
            ),
            Entry {
                title: "TITLE1".into(),
                name: "NAME1".into(),
                categories: vec!["CATEGORY1".into(), "CATEGORY2".into()],
                content: "CONTENT1".into(),
                updated: "2020-02-07T23:59:59Z".into(),
                draft: true,
            }
        )
    }

    #[test]
    fn new_dummy() {
        assert_eq!(
            Entry::new_dummy(),
            Entry {
                title: "TITLE".into(),
                name: "NAME".into(),
                categories: vec!["CATEGORY".into()],
                content: "CONTENT".into(),
                updated: "2020-02-07T00:00:00Z".into(),
                draft: true,
            }
        )
    }

    #[test]
    fn to_xml_simple() {
        let entry = Entry::new_dummy();
        assert_eq!(
            entry.to_xml(),
            r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <title>TITLE</title>
  <author><name>NAME</name></author>
  <content type="text/plain">CONTENT</content>
  <updated>2020-02-07T00:00:00Z</updated>
  <category term="CATEGORY" />
  <app:control>
    <app:draft>yes</app:draft>
  </app:control>
</entry>"#
        );
    }
}
