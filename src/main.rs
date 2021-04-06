extern crate dotenv;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use bson::{doc, oid};
use chrono::Utc;
use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::options::{ClientOptions, StreamAddress};
use mongodb::Client;
use serde::Deserialize;
use std::env;
use std::sync::*;

// =======Structs============
#[derive(Deserialize)]
struct Info {
    name: String,
}
#[derive(Deserialize)]
struct Delete {
    id: String,
}
#[derive(Deserialize)]
struct Update {
    id: String,
    name: String,
}
// =========================

async fn index(_req: HttpRequest) -> impl Responder {
    format!("Hello world")
}

async fn post_request(info: web::Json<Info>, data: web::Data<Mutex<Client>>) -> HttpResponse {
    let document = doc! {
        "name": info.name.to_string(),
        "created_at": Utc::now().timestamp_millis(),
        "updated_at": Utc::now().timestamp_millis(),
    };
    let collection = data.lock().unwrap().database("test1").collection("users");
    match collection.insert_one(document, None).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprintln!("Error while saving, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_request(req: HttpRequest, data: web::Data<Mutex<Client>>) -> HttpResponse {
    let oid = req.match_info().get("oid").expect("OID not found");
    let collection = data.lock().unwrap().database("test1").collection("users");
    match collection
        .find_one(
            doc! {"_id":bson::oid::ObjectId::with_string(oid).unwrap()},
            None,
        )
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprintln!("Error while getting document, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_all_request(_req: HttpRequest, data: web::Data<Mutex<Client>>) -> HttpResponse {
    let collection = data.lock().unwrap().database("test1").collection("users");
    let mut cursor = collection.find(None, None).await.expect("Find error");
    let mut vec = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(item) => vec.push(item),
            Err(e) => panic!("{}", e),
        }
    }
    HttpResponse::Ok().json(vec)
}

async fn update_doc(body: web::Json<Update>, data: web::Data<Mutex<Client>>) -> HttpResponse {
    let collection = data.lock().unwrap().database("test1").collection("users");
    let query = doc! {
        "_id": oid::ObjectId::with_string(&body.id).unwrap()
    };
    let update = doc! {
        "$set": {"name": &body.name,"updated_at": Utc::now().timestamp_millis(), }
    };
    match collection.update_one(query, update, None).await {
        Ok(update_result) => HttpResponse::Ok().json(update_result),
        Err(e) => {
            eprintln!("Error while updating document, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn delete_doc(info: web::Json<Delete>, data: web::Data<Mutex<Client>>) -> HttpResponse {
    let collection = data.lock().unwrap().database("test1").collection("users");
    let query = doc! {
        "_id": oid::ObjectId::with_string(&info.id).unwrap()
    };
    match collection.delete_many(query, None).await {
        Ok(delete_query) => HttpResponse::Ok().json(delete_query),
        Err(e) => {
            eprintln!("Error while updating document, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
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

    let client = web::Data::new(Mutex::new(Client::with_options(options).unwrap()));

    let port = env::var("PORT").expect("PORT not set.");
    let binding_address = get_server_address();
    println!("Server running at http://127.0.0.1:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .route("/", web::get().to(index))
            .route("/user/postdata", web::post().to(post_request))
            .route("/user/find/{oid}", web::get().to(get_request))
            .route("/user/getall", web::get().to(get_all_request))
            .route("/user/update", web::patch().to(update_doc))
            .route("/user/delete", web::delete().to(delete_doc))
    })
    .bind(&binding_address)?
    .run()
    .await
}
