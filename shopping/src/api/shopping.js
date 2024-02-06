const ShoppingService = require("../services/shopping-service");
const UserAuth = require("./middlewares/auth");
const { SubscribeMessage } = require("../utils");

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

  // Wishlist
  app.post("/wishlist", UserAuth, async (req, res, next) => {
    const { _id } = req.user;
    const { product_id } = req.body;

    const data = await service.addToWishlist(_id, product_id);
    return res.status(200).json(data);
  });

  app.get("/wishlist", UserAuth, async (req, res, next) => {
    const { _id } = req.user;
    const data = await service.getWishlist(_id);
    return res.status(200).json(data);
  });

  app.delete("/wishlist/:id", UserAuth, async (req, res, next) => {
    const { _id } = req.user;
    const product_id = req.params.id;

    const data = await service.removeFromWishlist(_id, product_id);
    return res.status(200).json(data);
  });

  // Orders
  app.post("/order", UserAuth, async (req, res, next) => {
    const { _id } = req.user;
    const { txnNumber } = req.body;

    try {
      const data = await service.createOrder(_id, txnNumber);
      return res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });

  app.get("/order/:id", UserAuth, async (req, res, next) => {
    const { _id } = req.user;

    try {
      const data = await service.GetOrder(_id);
      return res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });

  app.get("/orders", UserAuth, async (req, res, next) => {
    const { _id } = req.user;

    try {
      const data = await service.GetOrders(_id);
      return res.status(200).json(data);
    } catch (err) {
      next(err);
    }
  });
};
