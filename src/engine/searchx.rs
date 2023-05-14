//!
//! @file: searchx.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use crate::results_handler::aggregation_model::RawSearchResult;
use reqwest::header::{HeaderMap, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use std::collections::HashMap;
