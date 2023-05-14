//!
//! @file: parser.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use super::parser_model::Style;
use rlua::Lua;
use std::fs;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub binding_ip_addr: String,
    pub style: Style,
    pub redis_connection_url: String,
}

impl Config {
    pub fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let lua = Lua::new();

        lua.context(|context| {
            let globals = context.globals();

            context.load(&fs::read_to_string("./config.lua")?).exec()?;

            Ok(Config {
                port: globals.get::<_, u16>("port")?,
                binding_ip_addr: globals.get::<_, String>("binding_ip_addr")?,
                style: Style::new(
                    globals.get::<_, String>("theme")?,
                    globals.get::<_, String>("colorscheme")?,
                ),
                redis_connection_url: globals.get::<_, String>("redis_connection_url")?,
            })
        })
    }
}
