pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
}

pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
    pub description: String,
    pub is_operational: Option<bool>,
    pub error_stack: Option<bool>,
    pub logging_error_message: Option<String>,
}

impl AppError {
    pub fn new(message: String, status_code: StatusCode, description: String) -> AppError {
        AppError {
            message,
            status_code,
            description,
            is_operational: None,
            error_stack: None,
            logging_error_message: None,
        }
    }
}

// API ERRORS
pub struct APIError {
    pub app_error: AppError,
}

impl APIError {
    pub fn new(message: String) -> Self {
        APIError {
            app_error: AppError {
                message,
                status_code: StatusCode::InternalServerError,
                description: "Internal Server Error".to_string(),
                is_operational: Some(false),
                error_stack: Some(false),
                logging_error_message: None,
            },
        }
    }
}

// BAD REQUEST ERRORS
pub struct BadRequestError {
    pub app_error: AppError,
}

impl BadRequestError {
    pub fn new(&mut self, logging_error_response: String, message: String) -> &mut Self {
        self.app_error.message = message;
        self.app_error.status_code = StatusCode::BadRequest;
        self.app_error.description = "Bad Request".to_string();
        self.app_error.is_operational = Some(true);
        self.app_error.error_stack = Some(false);
        self.app_error.logging_error_message = Some(logging_error_response);

        self
    }
}

// VALIDATION ERRORS
pub struct ValidationError {
    pub app_error: AppError,
}

impl ValidationError {
    pub fn new(&mut self, error_stack: bool) -> &mut Self {
        self.app_error.message = "Bad Request".to_string();
        self.app_error.status_code = StatusCode::BadRequest;
        self.app_error.description = "Validation Error".to_string();
        self.app_error.is_operational = Some(true);
        self.app_error.error_stack = Some(error_stack);

        self
    }
}
