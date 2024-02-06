const Sentry = require("@sentry/node");
const _ = require("@sentry/tracing");
const {
  NotFoundError,
  ValidationError,
  AuthorizationError
} = require("./app-errors");

Sentry.init({
  dsn: process.env.SENTRY_DSN,

  // Set tracesSampleRate to 1.0 to capture 100%
  // of transactions for performance monitoring.
  // We recommend adjusting this value in production
  tracesSampleRate: 1.0
});

module.exports = (app) => {
  // catch all errors and report to logger
  app.use((error, req, res, next) => {
    let reportError = true;

    // skip common errors
    [NotFoundError, ValidationError, AuthorizationError].forEach(
      (typeOfError) => {
        if (error instanceof typeOfError) {
          reportError = false;
        }
      }
    );

    if (reportError) {
      Sentry.captureException(error);
    }

    const statusCode = error.statusCode || 500;
    const data = error.data || error.message;

    return res.status(statusCode).json({ data });
  });
};
