use std::convert::TryFrom;
use std::str::FromStr;

use crate::config::Config;
use crate::entry::{get_id, Entry};
use crate::{EntryId, EntryParams};
use atom_syndication::Feed;
use reqwest::{Method, Response, StatusCode, Url};
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

#[derive(Debug)]
struct HatenaBlogResponse {
    response: Response,
}

impl HatenaBlogResponse {
    async fn into_entry(self) -> Result<Entry, ClientError> {
        let body = self.response.text().await?;
        Entry::from_entry_xml(body.as_str()).map_err(|_| ClientError::ResponseBody)
    }

    async fn into_partial_list(self) -> Result<PartialList, ClientError> {
        let body = self.response.text().await?;
        let feed = Feed::from_str(body.as_str()).map_err(|_| ClientError::ResponseBody)?;
        Ok((
            feed.links
                .iter()
                .find(|link| link.rel == "next")
                .and_then(|link| Url::parse(link.href.as_str()).ok())
                .and_then(|href| {
                    href.query_pairs()
                        .into_iter()
                        .find(|(name, _)| name == "page")
                        .map(|(_, value)| value.to_string())
                }),
            feed.entries
                .iter()
                .map(|entry| get_id(entry).ok_or(ClientError::ResponseBody))
                .collect::<Result<Vec<EntryId>, ClientError>>()?,
        ))
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

type PartialList = (Option<String>, Vec<EntryId>);

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn create_entry(&self, entry_params: EntryParams) -> Result<Entry, ClientError> {
        let body = entry_params.into_xml();
        self.request(Method::POST, &self.collection_uri(None), Some(body))
            .await?
            .into_entry()
            .await
    }

    pub async fn delete_entry(&self, entry_id: &EntryId) -> Result<(), ClientError> {
        self.request(Method::DELETE, &self.member_uri(entry_id), None)
            .await
            .map(|_| ())
    }

    pub async fn get_entry(&self, entry_id: &EntryId) -> Result<Entry, ClientError> {
        self.request(Method::GET, &self.member_uri(entry_id), None)
            .await?
            .into_entry()
            .await
    }

    pub async fn list_entries_in_page(
        &self,
        page: Option<&str>,
    ) -> Result<PartialList, ClientError> {
        self.request(Method::GET, &self.collection_uri(page), None)
            .await?
            .into_partial_list()
            .await
    }

    pub async fn update_entry(
        &self,
        entry_id: &EntryId,
        entry_params: EntryParams,
    ) -> Result<Entry, ClientError> {
        let body = entry_params.into_xml();
        self.request(Method::PUT, &self.member_uri(entry_id), Some(body))
            .await?
            .into_entry()
            .await
    }

    fn collection_uri(&self, page: Option<&str>) -> String {
        let config = &self.config;
        format!(
            "https://blog.hatena.ne.jp/{}/{}/atom/entry{}",
            config.hatena_id,
            config.blog_id,
            page.map(|s| format!("?page={}", s))
                .unwrap_or_else(|| "".to_string())
        )
    }

    fn member_uri(&self, entry_id: &EntryId) -> String {
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
            client.collection_uri(None),
            "https://blog.hatena.ne.jp/HATENA_ID/BLOG_ID/atom/entry"
        )
    }

    #[test]
    fn member_uri() -> anyhow::Result<()> {
        let config = Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        let client = Client::new(&config);
        let entry_id = "ENTRY_ID".parse::<EntryId>()?;
        assert_eq!(
            client.member_uri(&entry_id),
            "https://blog.hatena.ne.jp/HATENA_ID/BLOG_ID/atom/entry/ENTRY_ID"
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
    fn update_entry() {
        // See: examples/update_entry.rs
    }
}
