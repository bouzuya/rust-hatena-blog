use std::str::FromStr;

use crate::config::Config;
use crate::entry::Entry;
use atom_syndication::Feed;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Client {
    config: Config,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
    #[error("response body error")]
    ResponseBody,
    #[error("bad request")]
    BadRequest,
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("method not allowed")]
    MethodNotAllowed,
    #[error("internal server error")]
    InternalServerError,
    #[error("unknown status code")]
    UnknownStatusCode,
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

fn get_id(entry: &atom_syndication::Entry) -> Option<String> {
    // https://blog.hatena.ne.jp/{HATENA_ID}/{BLOG_ID}/atom/entry/{ENTRY_ID}
    entry
        .links
        .iter()
        .find(|link| link.rel == "edit")
        .and_then(|link| link.href.split('/').last().map(|id| id.to_string()))
}

fn new_entry_from_entry_xml(body: String) -> Result<Entry, ClientError> {
    let xml = format!(
        "<feed>{}</feed>",
        body.strip_prefix(r#"<?xml version="1.0" encoding="utf-8"?>"#)
            .unwrap_or_else(|| body.as_str())
    );
    let feed = Feed::from_str(xml.as_str()).map_err(|_| ClientError::ResponseBody)?;
    let entry = feed.entries().first().ok_or(ClientError::ResponseBody)?;
    Ok(Entry::new(
        get_id(&entry).ok_or(ClientError::ResponseBody)?,
        entry.title.to_string(),
        entry
            .authors
            .first()
            .ok_or(ClientError::ResponseBody)?
            .name
            .to_string(),
        entry
            .categories
            .iter()
            .map(|c| c.term.clone())
            .collect::<Vec<String>>(),
        entry
            .content
            .clone()
            .ok_or(ClientError::ResponseBody)?
            .value
            .ok_or(ClientError::ResponseBody)?,
        entry.updated.to_rfc3339(),
        get_draft(&entry),
    ))
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn create_entry(
        &self,
        author_name: String,
        title: String,
        content: String,
        updated: String, // YYYY-MM-DDTHH:MM:SS
        categories: Vec<String>,
        draft: bool,
    ) -> Result<(), ClientError> {
        let config = &self.config;
        let client = reqwest::Client::new();
        let entry = Entry::new(
            "dummy".to_string(),
            title,
            author_name,
            categories,
            content,
            updated,
            draft,
        );
        let xml = entry.to_xml();
        let url = self.collection_uri();
        client
            .post(&url)
            .basic_auth(&config.hatena_id, Some(&config.api_key))
            .body(xml)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_entry(&self, entry_id: &str) -> Result<Entry, ClientError> {
        let config = &self.config;
        let client = reqwest::Client::new();
        let url = self.member_uri(entry_id);
        let response = client
            .get(&url)
            .basic_auth(&config.hatena_id, Some(&config.api_key))
            .send()
            .await?;
        match response.status() {
            status_code if status_code.is_success() => {
                let body = response.text().await?;
                new_entry_from_entry_xml(body)
            }
            StatusCode::BAD_REQUEST => Err(ClientError::BadRequest),
            StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ClientError::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => Err(ClientError::MethodNotAllowed),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ClientError::InternalServerError),
            _ => Err(ClientError::UnknownStatusCode),
        }
    }

    fn collection_uri(&self) -> String {
        let config = &self.config;
        format!(
            "https://blog.hatena.ne.jp/{}/{}/atom/entry",
            config.hatena_id, config.blog_id
        )
    }

    fn member_uri(&self, entry_id: &str) -> String {
        let config = &self.config;
        format!(
            "https://blog.hatena.ne.jp/{}/{}/atom/entry/{}",
            config.hatena_id, config.blog_id, entry_id,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let config = Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        assert_eq!(Client::new(&config), super::Client { config })
    }

    #[test]
    fn collection_uri() {
        let config = Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        let client = Client::new(&config);
        assert_eq!(
            client.collection_uri(),
            "https://blog.hatena.ne.jp/HATENA_ID/BLOG_ID/atom/entry"
        )
    }

    #[test]
    fn member_uri() {
        let config = Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        let client = Client::new(&config);
        assert_eq!(
            client.member_uri("ENTRY_ID"),
            "https://blog.hatena.ne.jp/HATENA_ID/BLOG_ID/atom/entry/ENTRY_ID"
        )
    }

    #[test]
    fn create_entry() {
        // TODO
        assert_eq!(1, 1);
    }

    #[test]
    fn get_entry() {
        // See: examples/get_entry.rs
    }
}
