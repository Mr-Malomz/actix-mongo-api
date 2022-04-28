use std::env;

use mongodb::{
    bson::doc,
    sync::{Client, Collection, Database},
};

use crate::models::user_model::User;

pub struct MongoRepo {}

impl MongoRepo {
    fn init() -> Collection<User> {
        let uri = match env::var("MONGOURI") {
            Ok(v) => v,
            Err(e) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        db.collection("User")
    }

    // fn getAllUsers(&self) -> Result<Vec<User>, &str> {
    //     let col = init();
    // }
}
