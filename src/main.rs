extern crate dotenv;
extern crate r2d2;
extern crate r2d2_mongodb;

use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

// =======Structs============
#[derive(Deserialize)]
struct Info {
    name: String,
}
// ==============

async fn with_id(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").expect("Name not found");
    let id = req.match_info().get("id").expect("ID not found");
    format!("Hello {} with id: {}! ", &name, &id)
}

async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello world")
}

async fn post_request(info: web::Json<Info>) -> HttpResponse {
    HttpResponse::Ok().body(format!("username: {}", info.name))
}

fn get_server_address() -> String {
    let port = env::var("PORT").expect("PORT not set.");
    return "127.0.0.1:".to_owned() + &port;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongodb_host = env::var("MONGODB_HOST").expect("MongoDB Host not found.");
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongodb_host, 27017)
            .with_db("test1")
            .build(),
    );

    let pool = Pool::builder().max_size(16).build(manager).unwrap();

    let port = env::var("PORT").expect("PORT not set.");
    let binding_address = get_server_address();
    println!("Server running at http://127.0.0.1:{}", port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{name}/{id}", web::get().to(with_id))
            .route("/postdata", web::post().to(post_request))
    })
    .bind(&binding_address)?
    .run()
    .await
}
