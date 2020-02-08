#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub api_key: String,
    pub blog_id: String,
    pub hatena_id: String,
}

impl Config {
    pub fn new(hatena_id: &str, blog_id: &str, api_key: &str) -> Self {
        Config {
            api_key: api_key.into(),
            blog_id: blog_id.into(),
            hatena_id: hatena_id.into(),
        }
    }

    pub fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = std::env::var("HATENA_API_KEY")?;
        let blog_id = std::env::var("HATENA_BLOG_ID")?;
        let hatena_id = std::env::var("HATENA_ID")?;
        Ok(Config::new(&hatena_id, &blog_id, &api_key))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn config_new() {
        assert_eq!(
            super::Config::new("HATENA_ID", "BLOG_ID", "API_KEY"),
            super::Config {
                api_key: "API_KEY".into(),
                blog_id: "BLOG_ID".into(),
                hatena_id: "HATENA_ID".into(),
            }
        );
    }

    #[test]
    fn config_new_from_env() {
        // TODO
        assert_eq!(1, 1);
    }
}
