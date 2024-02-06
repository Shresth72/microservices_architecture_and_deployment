const STATUS_CODES = {
  OK: 200,
  BAD_REQUEST: 400,
  UN_AUTHORISED: 403,
  NOT_FOUND: 404,
  INTERNAL_ERROR: 500,
};

class BaseError extends Error {
  constructor(
    name,
    statusCode,
    description,
  ) {
    super(description);
    Object.setPrototypeOf(this, new.target.prototype);
    this.name = name;
    this.statusCode = statusCode;
    Error.captureStackTrace(this);
  }
}

// 500 Internal Server Error
class APIError extends BaseError {
  constructor(description = "API Error") {
    super("API Internal Server Error", STATUS_CODES.INTERNAL_ERROR, description);
  }
}

// 400 Validation Error
class ValidationError extends BaseError {
  constructor(description = "BAD REQUEST") {
    super("BAD REQUEST", STATUS_CODES.BAD_REQUEST, description);
  }
}

// 403 Authorization Error
class AuthorizationError extends BaseError {
  constructor(description = "Access Denied") {
    super("Access Denied", STATUS_CODES.UN_AUTHORISED, description);
  }
}

// 404 Not Found
class NotFoundError extends BaseError {
  constructor(description = "Not Found") {
    super("Not Found", STATUS_CODES.NOT_FOUND, description);
  }
}
