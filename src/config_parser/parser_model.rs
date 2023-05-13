//!
//! @file: parser_model.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Style {
    pub theme: String,
    pub colorscheme: String,
}

impl Style {}
