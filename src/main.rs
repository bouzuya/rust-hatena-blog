mod config;
mod entry;

use config::Config;
use entry::Entry;

struct Client {
    config: Config,
}

impl Client {
    fn new(config: Config) -> Self {
        Self { config }
    }

    async fn create_entry(
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::new_from_env().expect("invalid env");
    let client = Client::new(config);
    let entry = Entry::new();
    client.create_entry(&entry).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn client_collection_uri() {
        let config = super::Config::new("HATENA_ID", "BLOG_ID", "API_KEY");
        let client = super::Client::new(config);
        assert_eq!(
            "https://blog.hatena.ne.jp/HATENA_ID/BLOG_ID/atom/entry",
            client.collection_uri()
        )
    }
}
