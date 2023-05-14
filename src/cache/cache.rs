//!
//! @file: cache.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use md5::compute;
use redis::{Client, Commands, Connection};

#[derive(Clone)]
pub struct RedisCache {
    redis_connection_url: String,
}

impl RedisCache {
    pub fn new(redis_connection_url: String) -> Self {
        RedisCache {
            redis_connection_url,
        }
    }

    fn compute_url_hash(self, url: &str) -> String {
        format!("{:?}", compute(url))
    }

    pub fn cached_results_json(self, url: String) -> Result<String, Box<dyn std::error::Error>> {
        let hashed_url_string = self.clone().compute_url_hash(&url);
        let mut redis_connection: Connection =
            Client::open(self.redis_connection_url)?.get_connection()?;
        Ok(redis_connection.get(hashed_url_string)?)
    }

    pub fn cache_results(
        self,
        json_results: String,
        url: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hashed_url_string = self.clone().compute_url_hash(&url);
        let mut redis_connection: Connection =
            Client::open(self.redis_connection_url)?.get_connection()?;

        redis_connection.set(hashed_url_string.clone(), json_results)?;

        redis_connection
            .expire::<String, u32>(hashed_url_string.clone(), 60)
            .unwrap();

        Ok(())
    }
}
