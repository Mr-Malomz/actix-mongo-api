use crate::{repository::mongodb_repo::MongoRepo};
use actix_web::{get, error, web::{Json, Data}, Responder, Result};

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> Result<impl Responder> {
    let users = db.get_all_users();

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(error::ErrorBadRequest("err getting list of users"))
    }
}