use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix::MailboxError;
use diesel::{r2d2::PoolError, result::{DatabaseErrorKind, Error as DieselError}};
// use jwt::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde_json::{Map as JsonMap, Value as JsonValue, json};
use validator::{Validate, ValidationError, ValidationErrors};
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    // 401 
    Unauthorized(JsonValue),

    // 403
    Forbidden(JsonValue),

    // 404
    NotFound(JsonValue),

    // 422
    UnprocessableEntity(JsonValue),

    // 500
    InternalServerError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        println!("WHAT THE ERROR ITSELF IS!!!!!!!!!!! {:#?}", self);
        write!(f, "{:#?}", self)
    }
}



impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Error::UnprocessableEntity(ref message) => HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message),
            Error::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}


impl From<MailboxError> for Error {
    fn from(_error: MailboxError) -> Self {
        Error::InternalServerError
    }
}


impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({"error": message}))
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({
                    "error": "requested record was not found"
                }))
            },
            // DieselError::RollbackTransaction => todo!(),
            // DieselError::AlreadyInTransaction => todo!(),
            _ => Error::InternalServerError,
        }
    }
}



// impl From<JwtError> for Error {
//     fn from(error: JwtError) -> Self {
//         match error.kind() {
//             JwtErrorKind::InvalidToken => Error::Unauthorized(json!({
//                 "error": "Invalid Jwt Token"
//             })),
//             JwtErrorKind::InvalidIssuer => Error::Unauthorized(json!({
//                 "error": "Issuer is invalid",
//             })),
//             _ => Error::Unauthorized(json!({
//                 "error": "An issue was found with the provided token",
//             }))
//         }
//     }
// }




impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}


impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        // transform errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors.iter().map(|error| {
                json!(error.message)
            }).collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::UnprocessableEntity(json!({
            "errors": err_map
        }))
    }
}