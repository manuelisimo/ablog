use actix_web::{web, ResponseError};
use actix_web::http::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use sailfish::RenderError;
use diesel::r2d2;

#[derive(Debug)]
pub struct BlogError {
    msg: String,
    status: u16,
}

impl BlogError {
    pub fn build(message: String, status: u16) -> Self {
        BlogError {
            msg: message,
            status,
        }
    }
}

impl Display for BlogError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg)
    }
}

impl ResponseError for BlogError {
    fn error_response(&self) -> web::HttpResponse {
        let body = self.msg.clone();
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .body(body)
    }
}

impl From<sailfish::RenderError> for BlogError {
    fn from(error: RenderError) -> Self {
        BlogError {
            msg: error.to_string(),
            status: 500,
        }
    }
}

impl From<diesel::result::Error> for BlogError {
    fn from(error: diesel::result::Error) -> Self {
        BlogError {
            msg: error.to_string(),
            status: 500,
        }
    }
}

impl From<r2d2::Error> for BlogError {
    fn from(error: r2d2::Error) -> Self {
        BlogError {
            msg: error.to_string(),
            status: 500,
        }
    }
}
