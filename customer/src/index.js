const express = require("express");
const { PORT } = require("./config");
const { databaseConnection } = require("./database");
const expressApp = require("./express-app");
const dotEnv = require("dotenv");
const { CreateChannel } = require("./utils");

const StartServer = async () => {
  dotEnv.config();

  const app = express();

  await databaseConnection();

  const channel = await CreateChannel();

  await expressApp(app, channel);

  // catch all errors and report to logger
  app.use((error, req, res, next) => {
    const statusCode = error.statusCode || 500;
    const data = error.data || error.message;

    return res.status(statusCode).json({ data });
  });

  app
    .listen(PORT, () => {
      console.log(`listening to port ${process.env.PORT}`);
    })
    .on("error", (err) => {
      console.log(err);
      process.exit();
    });
};

StartServer();
