use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    title: String,
    name: String, // author.name
    content: String,
    updated: String, // YYYY-MM-DDTHH:MM:SS
    categories: Vec<String>,
    draft: bool,
}

impl Entry {
    #[allow(dead_code)]
    pub fn new_dummy() -> Self {
        Self::new(
            "TITLE",
            "NAME",
            &vec!["CATEGORY"],
            "CONTENT",
            "2020-02-07T00:00:00Z",
            true,
        )
    }

    pub fn new(
        title: &str,
        name: &str,
        categories: &Vec<&str>,
        content: &str,
        updated: &str,
        draft: bool,
    ) -> Self {
        Entry {
            title: title.into(),
            name: name.into(),
            categories: categories.iter().map(|&s| s.into()).collect(),
            content: content.into(),
            updated: updated.into(),
            draft,
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
    #[test]
    fn new() {
        assert_eq!(
            super::Entry::new(
                "TITLE1",
                "NAME1",
                &vec!["CATEGORY1", "CATEGORY2"],
                "CONTENT1",
                "2020-02-07T23:59:59Z",
                true,
            ),
            super::Entry {
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
            super::Entry::new_dummy(),
            super::Entry {
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
        let entry = super::Entry::new_dummy();
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
