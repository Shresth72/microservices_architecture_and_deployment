const { OrderModel, CartModel, WishlistModel } = require("../models");
const { v4: uuidv4 } = require("uuid");
const { APIError, BadRequestError } = require("../../utils/app-errors");
const _ = require("lodash");

//Dealing with data base operations
class ShoppingRepository {
  async Cart(customerId) {
    try {
      return await CartModel.findOne({
        customerId
      });
    } catch (err) {
      throw APIError(
        "API Error",
        STATUS_CODES.INTERNAL_ERROR,
        "Unable to Find Category"
      );
    }
  }

  async Orders(customerId) {
    try {
      const orders = await OrderModel.find({ customerId });
      return orders;
    } catch (err) {
      throw APIError(
        "API Error",
        STATUS_CODES.INTERNAL_ERROR,
        "Unable to Find Orders"
      );
    }
  }

  async ManageCart(customerId, product, qty, isRemove) {
    try {
      const cart = await CartModel.findOne({ customerId });
      if (cart) {
        if (isRemove) {
          // remove item from cart
          const cartItems = _.filter(
            cart.items,
            (item) => item.product._id !== product._id
          );
          cart.items = cartItems;
        } else {
          // update cart
          const cartIndex = _.findIndex(cart.items, {
            product: { _id: product._id }
          });

          if (cartIndex > -1) {
            cart.items[cartIndex].unit = qty;
          } else {
            cart.items.push({ product: { ...product }, unit: qty });
          }
        }
        return await cart.save();
      } else {
        // create new cart
        return await CartModel.create({
          customerId,
          items: [{ product: { ...product }, unit: qty }]
        });
      }
    } catch (err) {
      throw APIError(
        "API Error",
        STATUS_CODES.INTERNAL_ERROR,
        "Unable to Find Category"
      );
    }
  }

  // Wishlist
  async ManageWishlist(customerId, product_id, isRemove) {
    try {
      const wishlist = await WishlistModel.findOne({ customerId });
      if (wishlist) {
        if (isRemove) {
          // remove products from wishlist
          const products = _.filter(
            wishlist.products,
            (product) => product._id !== product_id
          );
          wishlist.products = products;
        } else {
          // update wishlist
          const wishlistIndex = _.findIndex(wishlist.products, {
            _id: product_id
          });

          if (wishlistIndex < 0) {
            wishlist.items.push({ _id: product_id });
          }
        }
        return await wishlist.save();
      } else {
        // create a new one
        return await WishlistModel.create({
          customerId,
          products: [{ _id: product_id }]
        });
      }
    } catch (err) {
      throw APIError(
        "API Error",
        STATUS_CODES.INTERNAL_ERROR,
        "Unable to Find Category"
      );
    }
  }

  async getWishlistByCustomerId(customerId) {
    try {
      return await WishlistModel.findOne({ customerId });
    } catch (err) {
      throw APIError(
        "API Error",
        STATUS_CODES.INTERNAL_ERROR,
        "Unable to Find Category"
      );
    }
  }

  // Orders
  async CreateNewOrder(customerId, txnId) {
    //check transaction for payment Status

    try {
      const cart = await CartModel.findOne(customerId);

      if (cart) {
        let amount = 0;

        let cartItems = cart.items;

        if (cartItems.length > 0) {
          //process Order
          cartItems.map((item) => {
            amount += parseInt(item.product.price) * parseInt(item.unit);
          });

          const orderId = uuidv4();

          const order = new OrderModel({
            orderId,
            customerId,
            amount,
            txnId,
            status: "received",
            items: cartItems
          });

          cart.items = [];

          const orderResult = await order.save();

          profile.orders.push(orderResult);

          await cart.save();

          return orderResult;
        }
      }

      return {};
    } catch (err) {
      throw APIError(
        "API Error",
        STATUS_CODES.INTERNAL_ERROR,
        "Unable to Find Category"
      );
    }
  }
}

module.exports = ShoppingRepository;
