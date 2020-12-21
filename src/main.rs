extern crate dotenv;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use bson::doc;
use dotenv::dotenv;
use mongodb::options::{ClientOptions, StreamAddress};
use mongodb::Client;
use serde::Deserialize;
use std::env;

// =======Structs============
#[derive(Deserialize)]
struct Info {
    name: String,
}

pub struct State {
    client: mongodb::Client,
}
// ==============

async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello world")
}

async fn post_request(info: web::Json<Info>, data: web::Data<State>) -> impl Responder {
    let name: &str = &info.name;
    let collection = data.client.database("test1").collection("user");
    let result = collection
        .insert_one(doc! {"name":name}, None)
        .await
        .unwrap();
    HttpResponse::Ok().json(result).await
}

async fn get_request(req: HttpRequest, data: web::Data<State>) -> impl Responder {
    let oid = req.match_info().get("oid").expect("OID not found");
    let collection = data.client.database("test1").collection("user");
    let result = collection
        .find_one(
            doc! {"_id":bson::oid::ObjectId::with_string(oid).unwrap()},
            None,
        )
        .await
        .expect("Error in finding document");
    HttpResponse::Ok().json(result).await
}

fn get_server_address() -> String {
    let port = env::var("PORT").expect("PORT not set.");
    return "127.0.0.1:".to_owned() + &port;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongodb_host: String = env::var("MONGODB_HOST").expect("MongoDB Host not found.");

    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: mongodb_host,
            port: Some(27017),
        }])
        .build();

    let client = Client::with_options(options).unwrap();

    let port = env::var("PORT").expect("PORT not set.");
    let binding_address = get_server_address();
    println!("Server running at http://127.0.0.1:{}", port);
    HttpServer::new(move || {
        App::new()
            .data(State {
                client: client.clone(),
            })
            .route("/", web::get().to(index))
            .route("/postdata", web::post().to(post_request))
            .route("/user/find/{oid}", web::get().to(get_request))
    })
    .bind(&binding_address)?
    .run()
    .await
}
