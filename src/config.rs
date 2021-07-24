use std::env;

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub blog_id: String,
    pub hatena_id: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("not present")]
    InvalidVar,
}

impl Config {
    pub fn new(hatena_id: &str, base_url: Option<&str>, blog_id: &str, api_key: &str) -> Self {
        Config {
            api_key: api_key.into(),
            base_url: base_url
                .map(|s| s.to_string())
                .unwrap_or_else(|| "https://blog.hatena.ne.jp".to_string()),
            blog_id: blog_id.into(),
            hatena_id: hatena_id.into(),
        }
    }

    pub fn new_from_env() -> Result<Self, ConfigError> {
        let api_key = env::var("HATENA_API_KEY").map_err(|_| ConfigError::InvalidVar)?;
        let base_url = env::var("HATENA_BLOG_BASE_URL").ok();
        let blog_id = env::var("HATENA_BLOG_ID").map_err(|_| ConfigError::InvalidVar)?;
        let hatena_id = env::var("HATENA_ID").map_err(|_| ConfigError::InvalidVar)?;
        Ok(Config::new(
            &hatena_id,
            base_url.as_deref(),
            &blog_id,
            &api_key,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_new() {
        assert_eq!(
            Config::new("HATENA_ID", Some("BASE_URL"), "BLOG_ID", "API_KEY"),
            Config {
                api_key: "API_KEY".into(),
                base_url: "BASE_URL".into(),
                blog_id: "BLOG_ID".into(),
                hatena_id: "HATENA_ID".into(),
            }
        );
    }

    #[test]
    fn config_new_from_env() {
        let hatena_api_key = "hatena_api_key1";
        let hatena_blog_base_url = "hatena_blog_base_url1";
        let hatena_blog_id = "hatena_blog_id1";
        let hatena_id = "hatena_id1";
        env::set_var("HATENA_API_KEY", hatena_api_key);
        env::set_var("HATENA_BLOG_BASE_URL", hatena_blog_base_url);
        env::set_var("HATENA_BLOG_ID", hatena_blog_id);
        env::set_var("HATENA_ID", hatena_id);
        assert_eq!(
            Config::new_from_env().unwrap(),
            Config {
                api_key: hatena_api_key.to_string(),
                base_url: hatena_blog_base_url.to_string(),
                blog_id: hatena_blog_id.to_string(),
                hatena_id: hatena_id.to_string(),
            }
        );
    }
}
