use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("failed to parse")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("post failed")]
    PostError(#[from] reqwest::Error),

    #[error("unauthorized")]
    UnAuthorized,

    #[error("invalid payload field")]
    ReadPayloadError,
}

impl actix_web::ResponseError for MyError {}
