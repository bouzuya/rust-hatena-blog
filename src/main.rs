use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
struct Config {
    api_key: String,
    blog_id: String,
    hatena_id: String,
}

impl Config {
    fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = std::env::var("HATENA_API_KEY")?;
        let blog_id = std::env::var("HATENA_BLOG_ID")?;
        let hatena_id = std::env::var("HATENA_ID")?;
        Ok(Config {
            api_key,
            blog_id,
            hatena_id,
        })
    }
}

#[derive(Debug, Serialize)]
struct Entry {
    title: String,
    name: String, // author.name
    content: String,
    updated: String, // YYYY-MM-DDTHH:MM:SS
    categories: Vec<String>,
    draft: bool,
}

impl Entry {
    fn new() -> Self {
        Entry {
            title: "TITLE".into(),
            name: "NAME".into(),
            categories: vec!["CATEGORY".into()],
            content: "CONTENT".into(),
            updated: "2020-02-07T00:00:00Z".into(),
            draft: true,
        }
    }
}

fn entry_xml(entry: Entry) -> String {
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
            &json!(entry),
        )
        .expect("render_template")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    let entry = Entry::new();
    let xml = entry_xml(entry);
    println!("{}", xml);

    let text = reqwest::get("http://httpbin.org/ip").await?.text().await?;
    println!("Response: {}", text);

    let config = Config::new_from_env().expect("invalid env");
    let url = format!(
        "https://blog.hatena.ne.jp/{}/{}/atom/entry",
        config.hatena_id, config.blog_id
    );
    let client = reqwest::Client::new();
    client
        .post(&url)
        .basic_auth(config.hatena_id, Some(config.api_key))
        .body(xml)
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_entry_xml() {
        let entry = super::Entry::new();
        assert_eq!(
            super::entry_xml(entry),
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
