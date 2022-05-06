// use actix_web::{ResponseError, HttpResponse, http::header::ContentType};

// #[derive(Debug, std::fmt::Display)]
// pub enum UserError {
//     NotFound,
//     UpdateFailure,
//     CreateFailure,
//     BadRequest
// }

// impl ResponseError for UserError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code()).insert_header(ContentType::json()).body(self.error_response())
//     }
// }