use crate::{
    helper::error_helper::UserError, models::user_model::User, repository::mongodb_repo::MongoRepo,
};
use actix_web::{
    error, get, post, put,
    web::{Data, Json, Path},
    Responder, Result,
};
use mongodb::bson::oid::ObjectId;

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
pub async fn get_user(
    db: Data<MongoRepo>,
    path: Path<String>,
) -> Result<impl Responder, UserError> {
    let id = path.into_inner();
    if id.is_empty() {
        return Err(UserError::BadRequest);
    }
    let user_detail = db.get_user(&id);

    match user_detail {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(UserError::ServerError),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> Result<impl Responder, UserError> {
    let id = path.into_inner();
    if id.is_empty() {
        return Err(UserError::BadRequest);
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let update_result = db.update_user(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1  {
                let updated_user_info = db.get_user(&id);
                
               return  match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(UserError::ServerError),
                }
            } else {
                return Err(UserError::NotFound)
            }
        },
        Err(_) => Err(UserError::ServerError),
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
