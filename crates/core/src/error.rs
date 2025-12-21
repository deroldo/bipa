use derust::StatusCode;
use derust::httpx::{HttpError, HttpTags};
use serde_json::Value;
use std::error::Error;

pub trait CustomError {
    fn business(
        status_code: StatusCode,
        log_message: &str,
        tags: &HttpTags,
    ) -> Self;

    fn business_data(
        status_code: StatusCode,
        log_message: &str,
        json: &Value,
        tags: &HttpTags,
    ) -> Self;

    fn status(
        status_code: StatusCode,
        error: Box<dyn Error>,
        tags: &HttpTags,
    ) -> Self;

    fn unexpected(
        error: Box<dyn Error>,
        tags: &HttpTags,
    ) -> Self;
}

impl CustomError for HttpError {
    fn business(
        status_code: StatusCode,
        log_message: &str,
        tags: &HttpTags,
    ) -> Self {
        HttpError::without_body(status_code, log_message.to_string(), tags.clone())
    }

    fn business_data(
        status_code: StatusCode,
        log_message: &str,
        json: &Value,
        tags: &HttpTags,
    ) -> Self {
        HttpError::with_json(status_code, log_message.to_string(), json.clone(), tags.clone())
    }

    fn status(
        status_code: StatusCode,
        error: Box<dyn Error>,
        tags: &HttpTags,
    ) -> Self {
        HttpError::without_body(status_code, error.to_string(), tags.clone())
    }

    fn unexpected(
        error: Box<dyn Error>,
        tags: &HttpTags,
    ) -> Self {
        HttpError::without_body(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), tags.clone())
    }
}
