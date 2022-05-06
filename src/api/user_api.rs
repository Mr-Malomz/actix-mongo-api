use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    error, get, post,
    web::{Data, Json, Path},
    Responder, Result,
};

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> Result<impl Responder> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user = db.create_user(data);

    match user {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(error::ErrorBadRequest("err getting list of users")),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    let user_detail = db.get_user(id);

    match user_detail {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(error::ErrorBadRequest("err getting user detail")),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> Result<impl Responder> {
    let users = db.get_all_users();

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(error::ErrorBadRequest("err getting list of users")),
    }
}
