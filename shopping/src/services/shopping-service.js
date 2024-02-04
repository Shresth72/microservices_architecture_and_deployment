const { ShoppingRepository } = require("../database");
const { FormateData, RPCRequest } = require("../utils");

// All Business logic will be here
class ShoppingService {
  constructor() {
    this.repository = new ShoppingRepository();
  }

  /* ------------ Cart Interaction with RPC ------------- */
  async addCartItem(customerId, product_id, qty) {
    // Grab product info from the product service through RPC
    const productResponse = await RPCRequest("PRODUCT_RPC", {
      type: "VIEW_PRODUCT",
      data: { product_id }
    });

    if (productResponse && productResponse._id) {
      const data = await this.repository.ManageCart(
        customerId,
        productResponse,
        qty,
        false
      );
    }

    throw new Error("Product not found");
  }

  async removeCartItem(customerId, productId) {
    return await this.repository.ManageCart(
      customerId,
      { _id: productId },
      0,
      true
    );
  }

  async getCart(_id) {
    try {
      return this.repository.Cart(_id);
    } catch (err) {
      throw new APIError("Data Not found", err);
    }
  }

  // Wishlist
  async addToWishlist(customerId, product_id) {
    try {
      return await this.repository.ManageWishlist(
        customerId,
        product_id,
        false
      );
    } catch (err) {
      throw new APIError("Cannot add to wishlist", err);
    }
  }

  async removeFromWishlist(customerId, product_id) {
    try {
      return await this.repository.ManageWishlist(customerId, product_id, true);
    } catch (err) {
      throw new APIError("Cannot remove from wishlist", err);
    }
  }

  async getWishlist(customerId) {
    try {
      // Perform RPC call to get product details
      const { products } = await this.repository.getWishlistByCustomerId(
        customerId
      );

      if (Array.isArray(products)) {
        const ids = products.map(({ _id }) => _id);
        const productResponse = await RPCRequest("PRODUCT_RPC", {
          type: "VIEW_PRODUCTS",
          data: ids
        });

        if (productResponse) {
          return productResponse;
        } 
        return FormateData({ error: "No products found" });
      }
    } catch (err) {
      throw new APIError("Data Not found", err);
    }
  }

  // Orders
  async PlaceOrder(userInput) {
    const { _id, txnNumber } = userInput;

    // Verify the txn number with payment logs

    try {
      const orderResult = await this.repository.CreateNewOrder(_id, txnNumber);
      return FormateData(orderResult);
    } catch (err) {
      throw new APIError("Data Not found", err);
    }
  }

  async GetOrders(customerId) {
    try {
      const orders = await this.repository.Orders(customerId);
      return FormateData(orders);
    } catch (err) {
      throw new APIError("Data Not found", err);
    }
  }

  async ManageCart(customerId, item, qty, isRemove) {
    try {
      const cartResult = await this.repository.AddCartItem(
        customerId,
        item,
        qty,
        isRemove
      );

      return FormateData(cartResult);
    } catch (err) {
      throw new APIError("Data Not found", err);
    }
  }

  async SubscribeEvents(payload) {
    payload = JSON.parse(payload);

    const { event, data } = payload;

    const { userId, product, qty } = data;

    switch (event) {
      case "ADD_TO_CART":
        this.ManageCart(userId, product, qty, false);
        break;
      case "REMOVE_FROM_CART":
        this.ManageCart(userId, product, qty, true);
        break;
      case "DELETE_PROFILE":
        this.repository.ManageCart(userId, null, 0, true);
        break;
      default:
        break;
    }
  }

  async GetOrderPayload(userId, order, event) {
    if (order) {
      const payload = {
        event,
        data: { userId, order }
      };
      return FormateData(payload);
    }
    return FormateData({ error: "No order available" });
  }
}

module.exports = ShoppingService;
