//!
//! @file: aggregator.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use std::{collections::HashMap, time::Duration};

use rand::Rng;
use tokio::join;

use super::{
    aggregation_model::{RawSearchResult, SearchResult, SearchResults},
    user_agent::random_user_agent,
};
use crate::engine::{duckduckgo, searchx};

pub async fn aggregate(
    query: &str,
    page: u32,
) -> Result<SearchResults, Box<dyn std::error::Error>> {
    let user_agent: String = random_user_agent();
    let mut result_map: HashMap<String, RawSearchResult> = HashMap::new();

    let mut rng = rand::thread_rng();
    let delay_secs = rng.gen_range(1..10);
    std::thread::sleep(Duration::from_secs(delay_secs));

    let (ddg_map_results, searx_map_results) = join!(
        duckduckgo::results(query, page, &user_agent),
        searchx::results(query, page, &user_agent)
    );

    let ddg_map_results: HashMap<String, RawSearchResult> = ddg_map_results?;
    let searx_map_results: HashMap<String, RawSearchResult> = searx_map_results?;

    result_map.extend(ddg_map_results);

    searx_map_results.into_iter().for_each(|(key, value)| {
        result_map
            .entry(key)
            .and_modify(|result| {
                result.add_engines(value.clone().engine());
            })
            .or_insert_with(|| -> RawSearchResult {
                RawSearchResult::new(
                    value.title.clone(),
                    value.visiting_url.clone(),
                    value.description.clone(),
                    value.engine.clone(),
                )
            });
    });

    Ok(SearchResults::new(
        result_map
            .into_iter()
            .map(|(key, value)| {
                SearchResult::new(
                    value.title,
                    value.visiting_url,
                    key,
                    value.description,
                    value.engine,
                )
            })
            .collect(),
        query.to_string(),
    ))
}
