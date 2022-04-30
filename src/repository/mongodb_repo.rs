use std::env;

use mongodb::{
    sync::{Client, Collection }, bson::extjson::de::Error,
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>
}

impl MongoRepo {
    pub fn init() -> Self {
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        
        MongoRepo { col }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).ok().expect("error getting list of docs");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }

    // fn get_user() {
        
    // }
}
