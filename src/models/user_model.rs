use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: ObjectId,
    pub name: String,
    pub location: String,
    pub title: String,
}
