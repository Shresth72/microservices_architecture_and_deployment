const ShoppingService = require("../services/shopping-service");
const UserAuth = require("./middlewares/auth");
const { PublishMessage, SubscribeMessage } = require("../utils");

module.exports = (app) => {
  const service = new ShoppingService();
  SubscribeMessage(channel, service);

  // Cart
  app.get("/cart", UserAuth, async (req, res, next) => {
    const { _id } = req.user;
    try {
      const data = await service.getCart(_id);
      return res.status(200).json(data.cart);
    } catch (err) {
      next(err);
    }
  });

  app.delete("/cart/:id", UserAuth, async (req, res, next) => {
    try {
      const { _id } = req.user;
      const product_id = req.params.id;
      const { data } = await service.removeCartItem(_id, product_id);

      res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });

  app.post("/cart", UserAuth, async (req, res, next) => {
    try {
      const { _id } = req.user;
      const { product_id, qty } = req.body;
      const { data } = await service.addCartItem(_id, product_id, qty);

      res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });

  // Orders
  app.post("/order", UserAuth, async (req, res, next) => {
    const { _id } = req.user;
    const { txnNumber } = req.body;

    try {
      const { data } = await service.PlaceOrder({ _id, txnNumber });

      const payload = await service.GetOrderPayload(_id, data, "CREATE_ORDER");
      // PublishCustomerEvent(payload);
      PublishMessage(
        channel,
        process.env.CUSTOMER_BINDING_KEY,
        JSON.stringify(payload)
      );

      return res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });

  app.get("/orders", UserAuth, async (req, res, next) => {
    const { _id } = req.user;

    try {
      const { data } = service.GetOrders(_id);
      return res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });
};
