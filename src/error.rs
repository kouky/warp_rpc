use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceError {
    pub status: Status,
    pub message: Option<String>,
    pub cause: Option<String>,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match &self.message {
            None => "".to_string(),
            Some(str) => format!(" message={:?}", str),
        };
        let cause = match &self.cause {
            None => "".to_string(),
            Some(str) => format!(" cause={:?}", str),
        };
        return write!(f, "{} status=\"{}\" {} {}",
                      std::any::type_name::<Self>(),
                      self.status,
                      message,
                      cause);
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    VersionConflict,
    RequestTooLarge,
    TooManyRequests,
    DataCorruption,
    ServerError,
    NotImplemented,
    ServiceUnavailable,
    NotSpecified,
}

impl From<u16> for Status {
    fn from(code: u16) -> Self {
        match code {
            400 => Status::BadRequest,
            401 => Status::Unauthorized,
            403 => Status::Forbidden,
            404 => Status::NotFound,
            405 => Status::MethodNotAllowed,
            409 => Status::VersionConflict,
            413 => Status::RequestTooLarge,
            429 => Status::TooManyRequests,
            500 => Status::ServerError,
            501 => Status::NotImplemented,
            503 => Status::ServiceUnavailable,
            _ => Status::NotSpecified
        }
    }
}

impl Status {
    fn message(&self) -> &str {
        match self {
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::VersionConflict => "Version Conflict",
            Self::RequestTooLarge => "Request Too Large",
            Self::TooManyRequests => "Too Many Requests",
            Self::DataCorruption => "Data Corruption",
            Self::ServerError => "Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::NotSpecified => "Not Specified",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}", self.message());
    }
}

impl std::error::Error for ServiceError {}
