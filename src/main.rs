use crate::config::database::get_database;
use crate::routes::item_routes::init_item_routes;
use crate::state::app_state::AppState;
use actix_contrib_logger::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use http::StatusCode;
use log::Level;
extern crate dotenv;
use dotenv::dotenv;
use std::{env, i32};

mod config;
mod handlers;
mod models;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port_string = env::var("PORT").expect("PORT not set.");
    let port = port_string.parse::<u16>().unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize the database connection
    let db = match get_database().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            std::process::exit(1);
        }
    };

    // Create shared state
    let app_state = web::Data::new(AppState { db });

    // Start the Actix Web server
    HttpServer::new(move || {
        let logger = Logger::default().custom_level(|status| {
            if status.is_server_error() {
                Level::Error
            } else {
                Level::Info
            }
        });
        App::new()
            .wrap(logger)
            .app_data(app_state.clone())
            .configure(init_item_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
