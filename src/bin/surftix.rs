//!
//! @file: routes.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!
//!

use env_logger::Env;
use std::net::TcpListener;
use surftix::config_parser::parser::Config;
use surftix::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::parse().unwrap();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("started server on port {}", config.port);

    let listener = TcpListener::bind((config.binding_ip_addr.clone(), config.port))?;

    run(listener, config)?.await
}
