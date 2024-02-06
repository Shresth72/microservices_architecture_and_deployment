const { ShoppingRepository } = require("../database");
const { FormateData, RPCRequest } = require("../utils");
const { APIError, NotFoundError } = require("../utils/errors/app-errors");

// All Business logic will be here
class ShoppingService {
  constructor() {
    this.repository = new ShoppingRepository();
  }

  /* ------------ Cart Interaction with RPC ------------- */
  async getCart(_id) {
    try {
      return this.repository.Cart(_id);
    } catch (err) {
      throw new APIError("error in fetching cart");
    }
  }

  async addCartItem(customerId, product_id, qty) {
    try {
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

      throw new NotFoundError("product not found");
    } catch (err) {
      throw new APIError("cannot add to cart");
    }
  }

  async removeCartItem(customerId, productId) {
    try {
      return await this.repository.ManageCart(
        customerId,
        { _id: productId },
        0,
        true
      );
    } catch (err) {
      throw new APIError("cannot remove from cart");
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
      throw new APIError("cannot add to wishlist");
    }
  }

  async removeFromWishlist(customerId, product_id) {
    try {
      return await this.repository.ManageWishlist(customerId, product_id, true);
    } catch (err) {
      throw new APIError("cannot remove from wishlist");
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
        
        throw new NotFoundError("products not found");
      }
    } catch (err) {
      throw new APIError("cannot fetch wishlist");
    }
  }

  // Orders
  async createOrder(customerId, txnNumber) {
    // Verify the txn number with payment logs

    try {
      return await this.repository.CreateNewOrder(customerId, txnNumber);
    } catch (err) {
      throw new APIError("cannot create order");
    }
  }

  async GetOrder(orderId) {
    try {
      return this.repository.Orders("", orderId);
    } catch (err) {
      throw new APIError("cannot fetch order");
    }
  }

  async GetOrders(customerId) {
    try {
      return this.repository.Orders(customerId, "");
    } catch (err) {
      throw new APIError("cannot fetch orders");
    }
  }

  // Utilities and connections
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
      throw new APIError("cannot manage cart");
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

  async deleteProfileData(customerId) {
    try {
      return this.repository.deleteProfileData(customerId);
    } catch (err) {
      throw new APIError("cannot delete profile");
    }
  }

  async SubscribeEvents(payload) {
    payload = JSON.parse(payload);
    const { event, data } = payload;

    switch (event) {
      case "DELETE_PROFILE":
        await this.deleteProfileData(data.userId);
        break;
      default:
        break;
    }
  }
}

module.exports = ShoppingService;
