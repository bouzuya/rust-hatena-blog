use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EntryParams {
    author_name: String,
    title: String,
    content: String,
    updated: String, // YYYY-MM-DDTHH:MM:SS
    categories: Vec<String>,
    draft: bool,
}

impl EntryParams {
    pub fn new(
        author_name: String,
        title: String,
        content: String,
        updated: String, // YYYY-MM-DDTHH:MM:SS
        categories: Vec<String>,
        draft: bool,
    ) -> Self {
        Self {
            author_name,
            title,
            content,
            updated,
            categories,
            draft,
        }
    }

    pub fn into_xml(self) -> String {
        let registry = Handlebars::new();
        registry
            .render_template(
                r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <title>{{title}}</title>
  <author><name>{{author_name}}</name></author>
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
mod tests {
    use super::*;

    fn new_dummy() -> EntryParams {
        EntryParams::new(
            "AUTHOR_NAME".to_string(),
            "TITLE".to_string(),
            "CONTENT".to_string(),
            "2020-02-07T00:00:00Z".to_string(),
            vec!["CATEGORY".to_string()],
            true,
        )
    }

    #[test]
    fn into_xml() {
        let entry = new_dummy();
        assert_eq!(
            entry.into_xml(),
            r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <title>TITLE</title>
  <author><name>AUTHOR_NAME</name></author>
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
