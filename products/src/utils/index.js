const { v4: uuid4 } = require("uuid");
const amqplib = require("amqplib");
const bcrypt = require("bcrypt");
const jwt = require("jsonwebtoken");
const amqplib = require("amqplib");

const { APP_SECRET } = require("../config");

//Utility functions
module.exports.GenerateSalt = async () => {
  return await bcrypt.genSalt();
};

module.exports.GeneratePassword = async (password, salt) => {
  return await bcrypt.hash(password, salt);
};

module.exports.ValidatePassword = async (
  enteredPassword,
  savedPassword,
  salt
) => {
  return (await this.GeneratePassword(enteredPassword, salt)) === savedPassword;
};

module.exports.GenerateSignature = async (payload) => {
  try {
    return await jwt.sign(payload, APP_SECRET, { expiresIn: "30d" });
  } catch (error) {
    console.log(error);
    return error;
  }
};

module.exports.ValidateSignature = async (req) => {
  try {
    const signature = req.get("Authorization");
    console.log(signature);
    const payload = await jwt.verify(signature.split(" ")[1], APP_SECRET);
    req.user = payload;
    return true;
  } catch (error) {
    console.log(error);
    return false;
  }
};

module.exports.FormateData = (data) => {
  if (data) {
    return { data };
  } else {
    throw new Error("Data Not found!");
  }
};

/* ------------ Message Broker ------------- */

// Create Channel
module.exports.CreateChannel = async (connection) => {
  try {
    const connection = await amqplib.connect(process.env.MESSAGE_BROKER_URL);
    const channel = await connection.createChannel();

    await channel.assertExchange(process.env.EXCHANGE_NAME, "direct", false);
    return channel;
  } catch (err) {
    console.log("Error in creating channel");
    throw new Error(err);
  }
};

// Publish Messages
module.exports.PublishMessage = async (channel, binding_key, message) => {
  try {
    await channel.publish(
      process.env.EXCHANGE_NAME,
      binding_key,
      Buffer.from(message)
    );
    console.log(`Sent event ${message}`);
  } catch (err) {
    console.log("Error in publishing message");
    throw new Error(err);
  }
};

// Subscribe Messages
module.exports.SubscribeMessage = async (channel, binding_key) => {
  const appQueue = await channel.assertQueue(process.env.QUEUE_NAME);

  channel.bindQueue(appQueue.queue, process.env.EXCHANGE_NAME, binding_key);

  channel.consume(appQueue.queue, (data) => {
    console.log(`Received event ${data.content.toString()}`);
    channel.ack(data);
  });
};

/* ------------ RPC Connections ------------- */

let amqplibConnection = null;

const getChannel = async () => {
  if (amqplibConnection === null) {
    amqplibConnection = await amqplib.connect("amqp://localhost");
  }
  return await amqplibConnection.createChannel();
};

const expensiveDBOperation = (payload, fakeResponse) => {
  console.log(payload);
  console.log(fakeResponse);

  return new Promise((res, rej) => {
    setTimeout(() => {
      res(fakeResponse);
    }, 9000);
  });
};

const RPCObserver = async (RPC_QUEUE_NAME, fakeResponse) => {
  const channel = await getChannel();
  await channel.assertQueue(RPC_QUEUE_NAME, {
    durable: false
  });
  channel.prefetch(1);
  channel.consume(
    RPC_QUEUE_NAME,
    async (msg) => {
      if (msg.content) {
        // DB Operation
        const payload = JSON.parse(msg.content.toString());
        const response = await expensiveDBOperation(payload, fakeResponse); // call fake DB operation

        channel.sendToQueue(
          msg.properties.replyTo,
          Buffer.from(JSON.stringify(response)),
          {
            correlationId: msg.properties.correlationId
          }
        );
        channel.ack(msg);
      }
    },
    {
      noAck: false
    }
  );
};

const requestData = async (RPC_QUEUE_NAME, requestPayload, uuid) => {
  try {
    const channel = await getChannel();

    const q = await channel.assertQueue("", { exclusive: true });

    channel.sendToQueue(
      RPC_QUEUE_NAME,
      Buffer.from(JSON.stringify(requestPayload)),
      {
        replyTo: q.queue,
        correlationId: uuid
      }
    );

    return new Promise((resolve, reject) => {
      // timeout n
      const timeout = setTimeout(() => {
        channel.close();
        resolve("API could not fullfil the request!");
      }, 8000);
      channel.consume(
        q.queue,
        (msg) => {
          if (msg.properties.correlationId == uuid) {
            resolve(JSON.parse(msg.content.toString()));
            clearTimeout(timeout);
          } else {
            reject("data Not found!");
          }
        },
        {
          noAck: true
        }
      );
    });
  } catch (error) {
    console.log(error);
    return "error";
  }
};

const RPCRequest = async (RPC_QUEUE_NAME, requestPayload) => {
  const uuid = uuid4(); // correlationId
  return await requestData(RPC_QUEUE_NAME, requestPayload, uuid);
};
