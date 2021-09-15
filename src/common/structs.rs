use bson::{doc, oid};
use serde::{Deserialize, Serialize};

// =======Structs============
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Info<'a> {
    pub name: &'a str,
    pub age: i32,
    // #[serde(serialize_with = "bson::serde_helpers::serialize_chrono_datetime_as_bson_datetime")]
    // created_at: DateTime<Utc>,
    pub created_at: i64,
    pub updated_at: i64,
}
#[derive(Deserialize)]
pub struct Payload {
    pub name: String,
    pub age: i32,
}
#[derive(Deserialize)]
pub struct Delete {
    pub id: String,
}
#[derive(Deserialize)]
pub struct Update {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct FindQuery {
    pub _id: oid::ObjectId,
}

#[derive(Debug)]
pub struct Database {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    pub error: bool,
    pub message: Option<String>,
    pub _id: Option<bson::Bson>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}
// =========================
