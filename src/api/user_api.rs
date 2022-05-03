use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    error, get, post,
    web::{Data, Json},
    Responder, Result,
};

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> Result<impl Responder> {
    let users = db.get_all_users();

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(error::ErrorBadRequest("err getting list of users")),
    }
}

// #[post("/user")]
// pub async fn create_user(db: Data<MongoRepo>, newUser: Json<User>) -> Result<impl Responder> {
//     let {} = newUser;
//     let user = db.create_user(newUser);
// }
