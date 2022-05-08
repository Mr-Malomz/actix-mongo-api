use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "The provided ID was not found found")]
    NotFound,
    #[display(fmt = "An internal error occurred. Please try again later")]
    ServerError,
    #[display(fmt = "Bad request. Provide the right ")]
    BadRequest,
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::NotFound => StatusCode::NOT_FOUND,
            UserError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
