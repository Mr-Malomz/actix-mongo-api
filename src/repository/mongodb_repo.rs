use std::env;
extern crate dotenv;

use actix_web::web::Path;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("error creating user");

        Ok(user)
    }

    pub fn get_user(&self, path: Path<String>) -> Result<User, Error> {
        let id = path.into_inner();
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("error getting user detail");

        Ok(user_detail.unwrap())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("error getting list of docs");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }

    // fn get_user() {
    // }
}
