use crate::config::Config;
use crate::entry::Entry;
use reqwest::{Response, StatusCode};
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

async fn new_response_from_reqwest_response(response: Response) -> Result<Entry, ClientError> {
    match response.status() {
        status_code if status_code.is_success() => {
            let body = response.text().await?;
            Entry::from_entry_xml(body.as_str()).map_err(|_| ClientError::ResponseBody)
        }
        StatusCode::BAD_REQUEST => Err(ClientError::BadRequest),
        StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
        StatusCode::NOT_FOUND => Err(ClientError::NotFound),
        StatusCode::METHOD_NOT_ALLOWED => Err(ClientError::MethodNotAllowed),
        StatusCode::INTERNAL_SERVER_ERROR => Err(ClientError::InternalServerError),
        _ => Err(ClientError::UnknownStatusCode),
    }
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
    ) -> Result<Entry, ClientError> {
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
        let xml = entry.to_create_request_body_xml();
        let url = self.collection_uri();
        let response = client
            .post(&url)
            .basic_auth(&config.hatena_id, Some(&config.api_key))
            .body(xml)
            .send()
            .await?;
        new_response_from_reqwest_response(response).await
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
        new_response_from_reqwest_response(response).await
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
        // See: examples/create_entry.rs
    }

    #[test]
    fn get_entry() {
        // See: examples/get_entry.rs
    }
}
