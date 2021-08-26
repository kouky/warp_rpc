use crate::error::{ServiceError, Status};

impl From<reqwest::Error> for ServiceError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_decode() || e.is_body() || e.is_builder() {
            return ServiceError {
                status: Status::DataCorruption,
                message: Some("reqwest http client error".to_string()),
                cause: Some(e.to_string()),
            };
        }

        match e.status() {
            Some(status_code) => {
                ServiceError {
                    status: status_code.as_u16().into(),
                    message: Some("reqwest http client error".to_string()),
                    cause: Some(e.to_string()),
                }
            }
            None => {
                ServiceError {
                    status: Status::NotSpecified,
                    message: Some("reqwest http client error".to_string()),
                    cause: Some(format!("{:?}", e)),
                }
            }
        }
    }
}