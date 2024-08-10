use actix_web::{web, HttpResponse};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;

use crate::models::item::Item;
use crate::state::app_state::AppState;

pub async fn health_check(state: web::Data<AppState>) -> HttpResponse {
    // Perform a simple ping operation to check the database connection
    let ping_result = state.db.run_command(doc! { "ping": 1 }).await;

    match ping_result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Err(_) => HttpResponse::InternalServerError()
            .json(serde_json::json!({ "success": false, "error": "Database connection failed" })),
    }
}

// Create
pub async fn create_item(state: web::Data<AppState>, item: web::Json<Item>) -> HttpResponse {
    let collection = state.db.collection("items");
    let new_item = Item {
        id: None,
        name: item.name.clone(),
        value: item.value,
    };

    match collection.insert_one(new_item).await {
        Ok(_) => HttpResponse::Ok().body("Item created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create item: {}", e)),
    }
}

// Read All
pub async fn get_items(state: web::Data<AppState>) -> HttpResponse {
    let collection = state.db.collection::<Item>("items");

    // Correctly specifying the type for the cursor
    let mut cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to fetch items"),
    };

    let mut items: Vec<Item> = Vec::new();

    while let Ok(Some(item)) = cursor.try_next().await {
        items.push(item);
    }

    HttpResponse::Ok().json(items)
}

// Read by ID
pub async fn get_item(state: web::Data<AppState>, item_id: web::Path<String>) -> HttpResponse {
    let collection = state.db.collection::<Item>("items");
    let id = mongodb::bson::oid::ObjectId::parse_str(&item_id.into_inner()).unwrap();
    let filter = doc! { "_id": id };

    match collection.find_one(filter).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Update
pub async fn update_item(
    state: web::Data<AppState>,
    item_id: web::Path<String>,
    updated_item: web::Json<Item>,
) -> HttpResponse {
    let collection = state.db.collection::<Item>("items");
    let id = mongodb::bson::oid::ObjectId::parse_str(&item_id.into_inner()).unwrap();
    let filter = doc! { "_id": id };
    let update = doc! { "$set": { "name": &updated_item.name, "value": updated_item.value } };

    match collection.update_one(filter, update).await {
        Ok(_) => HttpResponse::Ok().body("Item updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update item: {}", e)),
    }
}

// Delete
pub async fn delete_item(state: web::Data<AppState>, item_id: web::Path<String>) -> HttpResponse {
    let collection = state.db.collection::<Item>("items");
    let id = mongodb::bson::oid::ObjectId::parse_str(&item_id.into_inner()).unwrap();
    let filter = doc! { "_id": id };

    match collection.delete_one(filter).await {
        Ok(_) => HttpResponse::Ok().body("Item deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete item: {}", e)),
    }
}
