use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_all_users};
use repository::mongodb_repo::MongoRepo;

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let db = MongoRepo::init();
        let db_data = Data::new(db);
        App::new()
            .app_data(db_data)
            .service(get_all_users)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
