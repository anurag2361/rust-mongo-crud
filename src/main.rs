extern crate dotenv;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use bson::{doc, oid};
use chrono::prelude::*;
use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::options::{ClientOptions, ServerAddress};
use mongodb::{Client, Collection};
use std::env;

mod common;

async fn index(_req: HttpRequest) -> impl Responder {
    format!("Hello world")
}

async fn post_request(
    info: Result<web::Json<common::Payload>, actix_web::Error>,
    data: web::Data<Client>,
) -> HttpResponse {
    let typed_collection: Collection<common::Info> =
        data.database("test1").collection::<common::Info>("users");
    let newinfo = common::Info {
        name: &info.as_ref().unwrap().name.to_string(),
        age: info.as_ref().unwrap().age,
        created_at: Utc::now().timestamp_millis(),
        updated_at: Utc::now().timestamp_millis(),
    };
    match typed_collection.insert_one(newinfo, None).await {
        Ok(result) => {
            let res_data = common::Response {
                error: false,
                message: None,
                _id: Some(result.inserted_id),
                name: Some(info.as_ref().unwrap().name.to_string()),
                age: Some(info.as_ref().unwrap().age),
                created_at: Some(newinfo.created_at),
                updated_at: Some(newinfo.updated_at),
            };
            HttpResponse::Ok().json(res_data)
        }
        Err(e) => {
            eprintln!("Error while saving, {:?}", e);
            let res_data = common::Response {
                error: true,
                message: Some(e.to_string()),
                _id: None,
                name: None,
                age: None,
                created_at: None,
                updated_at: None,
            };
            HttpResponse::Ok().json(res_data)
        }
    }
}

async fn get_request(req: HttpRequest, data: web::Data<Client>) -> HttpResponse {
    let oidstring = req.match_info().get("oid").expect("OID not found");
    let collection: Collection<common::Response> = data
        .database("test1")
        .collection::<common::Response>("users");
    let query = common::FindQuery {
        _id: bson::oid::ObjectId::parse_str(&oidstring).unwrap(),
    };
    let serialized_doc = bson::ser::to_document(&query).unwrap();
    match collection.find_one(serialized_doc, None).await {
        Ok(result) => {
            if result.is_some() {
                HttpResponse::Ok().json(result)
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            eprintln!("Error while getting document, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_all_request(_req: HttpRequest, data: web::Data<Client>) -> HttpResponse {
    let typed_collection: Collection<common::Response> = data
        .database("test1")
        .collection::<common::Response>("users");
    let mut cursor = typed_collection.find(None, None).await.expect("Find error");
    let mut vec = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(item) => {
                vec.push(item);
            }
            Err(e) => {
                eprintln!("Error while getting document, {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(vec)
}

async fn update_doc(body: web::Json<common::Update>, data: web::Data<Client>) -> HttpResponse {
    let typed_collection: Collection<common::Response> = data
        .database("test1")
        .collection::<common::Response>("users");
    let filter = doc! {"_id": oid::ObjectId::parse_str(&body.id).unwrap()};
    match typed_collection.find_one(filter, None).await {
        Ok(result) => {
            if result.is_some() {
                let update = doc! { "$set": { "name": &body.name ,"updated_at": Utc::now().timestamp_millis() }};
                let id = doc! {"_id": oid::ObjectId::parse_str(&body.id).unwrap() };
                match typed_collection.update_one(id, update, None).await {
                    Ok(update_result) => HttpResponse::Ok().json(update_result),
                    Err(e) => {
                        eprintln!("Error while updating document, {:?}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            eprintln!("Error while getting document, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn delete_doc(info: web::Json<common::Delete>, data: web::Data<Client>) -> HttpResponse {
    let typed_collection: Collection<common::Response> = data
        .database("test1")
        .collection::<common::Response>("users");
    let query = common::FindQuery {
        _id: bson::oid::ObjectId::parse_str(&info.id).unwrap(),
    };
    let serialized_doc = bson::ser::to_document(&query).unwrap();
    match typed_collection.find_one(serialized_doc, None).await {
        Ok(result) => {
            if result.is_some() {
                let id = doc! {"_id":&result.unwrap()._id};
                match typed_collection.delete_one(id, None).await {
                    Ok(delete_result) => HttpResponse::Ok().json(delete_result),
                    Err(e) => {
                        eprintln!("Error while updating document, {:?}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            eprintln!("Error while getting document, {:?}", e);
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

    let databse_conn_urls = common::Database {
        host: env::var("MONGODB_HOST").expect("MongoDB Host not found."),
        port: env::var("MONGODB_PORT")
            .expect("Mongodb port not found")
            .parse::<u16>()
            .unwrap(),
    };

    let options = ClientOptions::builder()
        .hosts(vec![ServerAddress::Tcp {
            host: databse_conn_urls.host.to_string(),
            port: Some(databse_conn_urls.port),
        }])
        .build();

    let client =
        web::Data::new(Client::with_options(options).expect("Error in generating mongodb client"));

    let port = env::var("PORT").expect("PORT not set.");
    let binding_address = get_server_address();
    println!("Server running at http://127.0.0.1:{}", port);
    println!(
        "Database running at mongodb://{:?}:{:?}",
        &databse_conn_urls.host, databse_conn_urls.port
    );
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
    .await?;
    Ok(())
}
