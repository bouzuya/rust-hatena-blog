use std::convert::TryFrom;

use crate::config::Config;
use crate::entry::Entry;
use crate::EntryId;
use reqwest::{Method, Response, StatusCode};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Client {
    config: Config,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn into_entry(self, id: EntryId) -> Entry {
        Entry::new(
            id,
            self.title,
            self.author_name,
            self.categories,
            self.content,
            self.updated,
            self.draft,
        )
    }
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

#[derive(Debug)]
struct HatenaBlogResponse {
    response: Response,
}

impl HatenaBlogResponse {
    async fn into_entry(self) -> Result<Entry, ClientError> {
        let body = self.response.text().await?;
        Entry::from_entry_xml(body.as_str()).map_err(|_| ClientError::ResponseBody)
    }
}

impl TryFrom<Response> for HatenaBlogResponse {
    type Error = ClientError;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match response.status() {
            status_code if status_code.is_success() => Ok(Self { response }),
            StatusCode::BAD_REQUEST => Err(ClientError::BadRequest),
            StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ClientError::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => Err(ClientError::MethodNotAllowed),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ClientError::InternalServerError),
            _ => Err(ClientError::UnknownStatusCode),
        }
    }
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn create_entry(&self, entry_params: EntryParams) -> Result<Entry, ClientError> {
        // FIXME
        let entry = entry_params.into_entry("dummy".parse().unwrap());
        let xml = entry.to_request_body_xml();
        self.request(Method::POST, &self.collection_uri(), Some(xml))
            .await?
            .into_entry()
            .await
    }

    pub async fn delete_entry(&self, entry_id: &str) -> Result<(), ClientError> {
        self.request(Method::DELETE, &self.member_uri(entry_id), None)
            .await
            .map(|_| ())
    }

    pub async fn get_entry(&self, entry_id: &str) -> Result<Entry, ClientError> {
        self.request(Method::GET, &self.member_uri(entry_id), None)
            .await?
            .into_entry()
            .await
    }

    pub async fn update_entry(
        &self,
        entry_id: &str,
        entry_params: EntryParams,
    ) -> Result<Entry, ClientError> {
        // FIXME
        let entry = entry_params.into_entry("dummy".parse().unwrap());
        let xml = entry.to_request_body_xml();
        self.request(Method::PUT, &self.member_uri(entry_id), Some(xml))
            .await?
            .into_entry()
            .await
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

    async fn request(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
    ) -> Result<HatenaBlogResponse, ClientError> {
        let config = &self.config;
        let client = reqwest::Client::new();
        let request = client
            .request(method, url)
            .basic_auth(&config.hatena_id, Some(&config.api_key));
        let request = if let Some(body) = body {
            request.body(body)
        } else {
            request
        };
        let response = request.send().await?;
        HatenaBlogResponse::try_from(response)
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
    fn delete_entry() {
        // See: examples/delete_entry.rs
    }

    #[test]
    fn get_entry() {
        // See: examples/get_entry.rs
    }

    #[test]
    fn update_entry() {
        // See: examples/update_entry.rs
    }
}
