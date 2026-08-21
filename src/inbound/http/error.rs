use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html::from("Internal Server Error"),
                )
                    .into_response()
            }
        }
    }
}
