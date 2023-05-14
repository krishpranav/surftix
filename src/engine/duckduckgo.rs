//!
//! @file: duckduckgo.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use crate::results_handler::aggregation_model::RawSearchResult;
use reqwest::header::{HeaderMap, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use std::collections::HashMap;

pub async fn results(
    query: &str,
    page: u32,
    user_agent: &str,
) -> Result<HashMap<String, RawSearchResult>, Box<dyn std::error::Error>> {
    let url: String = match page {
        1 => {
            format!("https://html.duckduckgo.com/html/?q={query}&s=&dc=&v=1&o=json&api=/d.js")
        }
        _ => {
            format!(
                "https://duckduckgo.com/html/?q={}&s={}&dc={}&v=1&o=json&api=/d.js",
                query,
                (page / 2 + (page % 2)) * 30,
                (page / 2 + (page % 2)) * 30 + 1
            )
        }
    };

    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT, user_agent.parse()?);
    header_map.insert(REFERER, "https://google.com/".parse()?);
    header_map.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);
    header_map.insert(COOKIE, "kl=wt-wt".parse()?);

    let results: String = reqwest::Client::new()
        .get(url)
        .headers(header_map)
        .send()
        .await?
        .text()
        .await?;

    let document: Html = Html::parse_document(&results);
    let results: Selector = Selector::parse(".result")?;
    let result_title: Selector = Selector::parse(".result__a")?;
    let result_url: Selector = Selector::parse(".result__url")?;
    let result_desc: Selector = Selector::parse(".result__snippet")?;

    Ok(document
        .select(&results)
        .map(|result| {
            RawSearchResult::new(
                result
                    .select(&result_title)
                    .next()
                    .unwrap()
                    .inner_html()
                    .trim()
                    .to_string(),
                format!(
                    "https://{}",
                    result
                        .select(&result_url)
                        .next()
                        .unwrap()
                        .inner_html()
                        .trim()
                ),
                result
                    .select(&result_desc)
                    .next()
                    .unwrap()
                    .inner_html()
                    .trim()
                    .to_string(),
                vec!["duckduckgo".to_string()],
            )
        })
        .map(|search_result| (search_result.visiting_url.clone(), search_result))
        .collect())
}
