use crate::config::Config;
use crate::entry::Entry;

#[derive(Debug, Eq, PartialEq)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn create_entry(
        &self,
        entry: &Entry,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let config = &self.config;
        let client = reqwest::Client::new();
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

    fn collection_uri(&self) -> String {
        let config = &self.config;
        format!(
            "https://blog.hatena.ne.jp/{}/{}/atom/entry",
            config.hatena_id, config.blog_id
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn new() {
        let config = super::Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        assert_eq!(super::Client::new(&config), super::Client { config })
    }

    #[test]
    fn collection_uri() {
        let config = super::Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        let client = super::Client::new(&config);
        assert_eq!(
            client.collection_uri(),
            "https://blog.hatena.ne.jp/HATENA_ID/BLOG_ID/atom/entry"
        )
    }

    #[test]
    fn create_entry() {
        // TODO
        assert_eq!(1, 1);
    }
}
