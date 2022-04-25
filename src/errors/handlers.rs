// TODO: Possibly implement a "new function" as seen here: https://github.com/iamhabbeboy/rest-api-actix-web/blob/master/src/error_handler.rs

use actix_web::{
    error::Error, error::JsonPayloadError, error::PathError, error::ResponseError,
    http::StatusCode, HttpRequest, HttpResponse,
};
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use serde::Serialize;

// Struct which defines the structure of the error response
#[derive(Serialize)]
struct FormattedErrorResponse {
    status_code: u16,
    error: String,
    message: String,
}

// Custom data type for the error response
#[derive(Debug, Display, Error, Serialize)]
pub enum CustomError {
    // Formatting the validation error message
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    // Formatting the internatal server error error message
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    // Formatting the bad request error message
    #[display(fmt = "Bad request")]
    BadClientData,
    #[display(fmt = "Not found!")]
    NotFound,
}

// Implement function to return the error name
impl CustomError {
    fn name(&self) -> String {
        match self {
            CustomError::ValidationError { .. } => "Validation Error".to_string(),
            CustomError::InternalError => "Internal Server Error".to_string(),
            CustomError::BadClientData => "Bad request".to_string(),
            CustomError::NotFound => "Not found".to_string(),
        }
    }
}

// Implement ResponseError trait for the custom struct
impl ResponseError for CustomError {
    // Function to generate the error response
    fn error_response(&self) -> HttpResponse {
        let error_response = FormattedErrorResponse {
            status_code: self.status_code().as_u16(),
            error: self.to_string(),
            message: self.name(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
    // Function to generate the error code
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::BadClientData => StatusCode::BAD_REQUEST,
            CustomError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// Implementation of diesel errors in order to use the custom type for routes that interact with the database
impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> CustomError {
        match error {
            DieselError::DatabaseError(_, _err) => CustomError::InternalError,
            DieselError::NotFound => CustomError::NotFound,
            _err => CustomError::InternalError,
        }
    }
}

// Custom JSON error handler for the JSON deserialization
// TODO: See if a user friendly version of the exact error can be generated (E.g. invalid type: string "1", expected u32)
pub fn json_error_handler(_err: JsonPayloadError, _req: &HttpRequest) -> Error {
    CustomError::BadClientData.into()
}

// Custom Path error handler for when the provided type in the URL does not match the expected type
pub fn path_error_handler(_err: PathError, _req: &HttpRequest) -> Error {
    CustomError::BadClientData.into()
}
