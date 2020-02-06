use handlebars::Handlebars;
use serde_json::json;

fn entry_xml() -> String {
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
            &json!({
                "title": "TITLE",
                "name": "NAME",
                "categories": vec!["CATEGORY"],
                "content": "CONTENT",
                "updated": "2020-02-07T00:00:00Z",
                "draft": true
            }),
        )
        .expect("render_template")
}

fn main() {
    println!("Hello, world!");
    println!("{}", entry_xml());
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_entry_xml() {
        assert_eq!(
            super::entry_xml(),
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
