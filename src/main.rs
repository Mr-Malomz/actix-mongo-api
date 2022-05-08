mod api;
mod helper;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let db = MongoRepo::init();
        let db_data = Data::new(db);
        App::new()
            .app_data(db_data)
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
