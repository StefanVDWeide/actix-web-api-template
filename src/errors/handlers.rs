use actix_web::{
    error::Error, error::InternalError, error::JsonPayloadError, error::PathError,
    error::ResponseError, http::StatusCode, HttpRequest, HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;

// Struct which defines the structure of the error response
#[derive(Serialize)]
struct FormattedErrorResponse {
    status_code: u16,
    error: String,
    message: String,
}

// Custom data type for the error response
#[derive(Debug, Display, Error)]
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

// Custom JSON error handler for the JSON deserialization
// TODO: See if this can be made to work with the CustomerError type
pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    InternalError::from_response(err, HttpResponse::BadRequest().json("Invalid JSON payload"))
        .into()
}

// Custom Path error handler for when the provided type in the URL does not match the expected type
// TODO: See if this can be made to work with the CustomerError type
pub fn path_error_handler(err: PathError, _req: &HttpRequest) -> Error {
    InternalError::from_response(
        err,
        HttpResponse::BadRequest().json("Invalid path parameter"),
    )
    .into()
}
