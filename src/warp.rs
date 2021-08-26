use crate::error::Status;
use warp::http::StatusCode;

impl From<Status> for warp::http::StatusCode {
    fn from(status: Status) -> Self {
        match status {
            Status::BadRequest => StatusCode::BAD_REQUEST,
            Status::Unauthorized => StatusCode::UNAUTHORIZED,
            Status::Forbidden => StatusCode::FORBIDDEN,
            Status::NotFound => StatusCode::NOT_FOUND,
            Status::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Status::VersionConflict => StatusCode::CONFLICT,
            Status::RequestTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Status::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Status::DataCorruption => StatusCode::INTERNAL_SERVER_ERROR,
            Status::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Status::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            Status::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            Status::NotSpecified => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
