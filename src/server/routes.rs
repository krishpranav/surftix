//!
//! @file: routes.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use crate::{
    cache::cache::RedisCache,
    config_parser::parser::Config,
    results_handler::{aggregation_model::SearchResults, aggregator::aggregate},
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
struct SearchParams {
    q: Option<String>,
    page: Option<u32>,
}

#[get("/")]
pub async fn index(
    hbs: web::Data<Handlebars<'_>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String = hbs.render("index", &config.style).unwrap();
    Ok(HttpResponse::Ok().body(page_content))
}

pub async fn not_found(
    hbs: web::Data<Handlebars<'_>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String = hbs.render("404", &config.style)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page_content))
}

#[get("/search")]
pub async fn search(
    hbs: web::Data<Handlebars<'_>>,
    req: HttpRequest,
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let params = web::Query::<SearchParams>::from_query(req.query_string())?;

    let redis_cache = RedisCache::new(config.redis_connection_url.clone());
    match &params.q {
        Some(query) => {
            if query.trim().is_empty() {
                Ok(HttpResponse::Found()
                    .insert_header(("location", "/"))
                    .finish())
            } else {
                let mut page_url = String::new();

                let page = match params.page {
                    Some(page_number) => {
                        if page_number <= 1 {
                            page_url = format!(
                                "http://{}:{}/search?q={}&page={}",
                                config.binding_ip_addr, config.port, query, 1
                            );
                            1
                        } else {
                            page_url = format!(
                                "http://{}:{}/search?q={}&page={}",
                                config.binding_ip_addr, config.port, query, page_number
                            );

                            page_number
                        }
                    }
                    None => {
                        page_url = format!(
                            "http://{}:{}{}&page={}",
                            config.binding_ip_addr,
                            config.port,
                            req.uri(),
                            1
                        );

                        1
                    }
                };

                let cached_results_json = redis_cache.clone().cached_results_json(page_url.clone());
                match cached_results_json {
                    Ok(results_json) => {
                        let new_results_json: SearchResults = serde_json::from_str(&results_json)?;
                        let page_content: String = hbs.render("search", &new_results_json)?;
                        Ok(HttpResponse::Ok().body(page_content))
                    }
                    Err(_) => {
                        let mut results_json: crate::results_handler::aggregation_model::SearchResults =
                    aggregate(query, page).await?;
                        results_json.add_style(config.style.clone());
                        redis_cache.clone().cache_results(
                            serde_json::to_string(&results_json)?,
                            page_url.clone(),
                        )?;
                        let page_content: String = hbs.render("search", &results_json)?;
                        Ok(HttpResponse::Ok().body(page_content))
                    }
                }
            }
        }
        None => Ok(HttpResponse::Found()
            .insert_header(("location", "/"))
            .finish()),
    }
}

#[get("/robots.txt")]
pub async fn robots_data(_req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String = read_to_string("./public/robots.txt")?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=ascii")
        .body(page_content))
}

#[get("/about")]
pub async fn about(
    hbs: web::Data<Handlebars<'_>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String = hbs.render("about", &config.style)?;
    Ok(HttpResponse::Ok().body(page_content))
}

#[get("/settings")]
pub async fn settings(
    hbs: web::Data<Handlebars<'_>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String = hbs.render("settings", &config.style)?;
    Ok(HttpResponse::Ok().body(page_content))
}
