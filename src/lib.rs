pub mod cache;
pub mod config_parser;
pub mod engine;
pub mod results_handler;
pub mod server;

use std::net::TcpListener;

use crate::server::routes;

use actix_files as fs;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use config_parser::parser::Config;
use handlebars::Handlebars;

pub fn run(listener: TcpListener, config: Config) -> std::io::Result<Server> {
    let mut handlebars: Handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./frontend/templates")
        .unwrap();

    let handlebars_ref: web::Data<Handlebars> = web::Data::new(handlebars);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "./frontend/static").show_files_listing())
            .service(fs::Files::new("/images", "./frontend/assets").show_files_listing())
            .service(routes::robots_data)
            .service(routes::index)
            .service(routes::search)
            .service(routes::about)
            .service(routes::settings)
            .default_service(web::route().to(routes::not_found))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
