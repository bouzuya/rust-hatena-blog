use crate::Config;
use crate::EntryId;
use crate::EntryParams;
use crate::{
    CreateEntryResponse, DeleteEntryResponse, GetEntryResponse, ListCategoriesResponse,
    ListEntriesResponse, UpdateEntryResponse,
};
use reqwest::{Method, StatusCode};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Client {
    config: Config,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
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

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn create_entry(
        &self,
        entry_params: EntryParams,
    ) -> Result<CreateEntryResponse, ClientError> {
        let body = entry_params.into_xml();
        self.request(Method::POST, &self.collection_uri(None), Some(body))
            .await
            .map(CreateEntryResponse::from)
    }

    pub async fn delete_entry(
        &self,
        entry_id: &EntryId,
    ) -> Result<DeleteEntryResponse, ClientError> {
        self.request(Method::DELETE, &self.member_uri(entry_id), None)
            .await
            .map(DeleteEntryResponse::from)
    }

    pub async fn get_entry(&self, entry_id: &EntryId) -> Result<GetEntryResponse, ClientError> {
        self.request(Method::GET, &self.member_uri(entry_id), None)
            .await
            .map(GetEntryResponse::from)
    }

    pub async fn list_categories(&self) -> Result<ListCategoriesResponse, ClientError> {
        self.request(Method::GET, &self.category_document_uri(), None)
            .await
            .map(ListCategoriesResponse::from)
    }

    pub async fn list_entries_in_page(
        &self,
        page: Option<&str>,
    ) -> Result<ListEntriesResponse, ClientError> {
        self.request(Method::GET, &self.collection_uri(page), None)
            .await
            .map(ListEntriesResponse::from)
    }

    pub async fn update_entry(
        &self,
        entry_id: &EntryId,
        entry_params: EntryParams,
    ) -> Result<UpdateEntryResponse, ClientError> {
        let body = entry_params.into_xml();
        self.request(Method::PUT, &self.member_uri(entry_id), Some(body))
            .await
            .map(UpdateEntryResponse::from)
    }

    fn category_document_uri(&self) -> String {
        let config = &self.config;
        format!(
            "{}/{}/{}/atom/category",
            config.base_url, config.hatena_id, config.blog_id
        )
    }

    fn collection_uri(&self, page: Option<&str>) -> String {
        let config = &self.config;
        format!(
            "{}/{}/{}/atom/entry{}",
            config.base_url,
            config.hatena_id,
            config.blog_id,
            page.map(|s| format!("?page={}", s))
                .unwrap_or_else(|| "".to_string())
        )
    }

    fn member_uri(&self, entry_id: &EntryId) -> String {
        let config = &self.config;
        format!(
            "{}/{}/{}/atom/entry/{}",
            config.base_url, config.hatena_id, config.blog_id, entry_id,
        )
    }

    async fn request(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
    ) -> Result<String, ClientError> {
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
        match response.status() {
            status_code if status_code.is_success() => {
                let body = response.text().await?;
                Ok(body)
            }
            StatusCode::BAD_REQUEST => Err(ClientError::BadRequest),
            StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ClientError::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => Err(ClientError::MethodNotAllowed),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ClientError::InternalServerError),
            _ => Err(ClientError::UnknownStatusCode),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn config() -> Config {
        Config::new("HATENA_ID", Some("BASE_URL"), "BLOG_ID", "API_KEY")
    }

    #[test]
    fn new() {
        let config = config();
        assert_eq!(Client::new(&config), super::Client { config })
    }

    #[test]
    fn collection_uri() {
        let client = Client::new(&config());
        assert_eq!(
            client.collection_uri(None),
            "BASE_URL/HATENA_ID/BLOG_ID/atom/entry"
        )
    }

    #[test]
    fn member_uri() -> anyhow::Result<()> {
        let client = Client::new(&config());
        let entry_id = "ENTRY_ID".parse::<EntryId>()?;
        assert_eq!(
            client.member_uri(&entry_id),
            "BASE_URL/HATENA_ID/BLOG_ID/atom/entry/ENTRY_ID"
        );
        Ok(())
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
    fn list_categories() {
        // See: examples/list_categories.rs
    }

    #[test]
    fn list_entries_in_page() {
        // See: examples/list_entries.rs
    }

    #[test]
    fn update_entry() {
        // See: examples/update_entry.rs
    }
}
