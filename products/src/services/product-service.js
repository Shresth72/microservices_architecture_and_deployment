const { APIError, NotFoundError } = require("../utils/errors/app-errors");
const { ProductRepository } = require("../database");
const { FormateData } = require("../utils");

// All Business logic will be here
class ProductService {
  constructor() {
    this.repository = new ProductRepository();
  }

  async GetProducts() {
    try {
      const products = await this.repository.Products();

      if (!products) throw new NotFoundError("No products available");

      let categories = {};

      products.map(({ type }) => {
        categories[type] = type;
      });

      return {
        products,
        categories: Object.keys(categories)
      };
    } catch (err) {
      throw new APIError("error in fetching products");
    }
  }

  async CreateProduct(productInputs) {
    try {
      return this.repository.CreateProduct(productInputs);
    } catch (err) {
      throw new APIError("error in creating product");
    }
  }

  async GetProductsByCategory(category) {
    try {
      return this.repository.FindByCategory(category);
    } catch (err) {
      throw new APIError("error in fetching products by category");
    }
  }

  async GetProductDescription(productId) {
    try {
      return this.repository.FindById(productId);
    } catch (err) {
      throw new APIError("error in fetching product description");
    }
  }


  async GetSelectedProducts(selectedIds) {
    try {
      const products = await this.repository.FindSelectedProducts(selectedIds);
      return FormateData(products);
    } catch (err) {
      throw new APIError("Data Not found");
    }
  }

  async GetProductById(productId) {
    try {
      return await this.repository.FindById(productId);
    } catch (err) {
      throw new APIError("Data Not found");
    }
  }

  async GetProductPayload(userId, { productId, qty }, event) {
    const product = await this.repository.FindById(productId);

    if (product) {
      const payload = {
        event,
        data: { userId, product, qty }
      };
      return FormateData(payload);
    }
    return FormateData({ error: "No product available" });
  }

  // RPC Response
  async serveRPCRequest(payload) {
    const { type, data } = payload;
    switch (type) {
      case "VIEW_PRODUCT":
        return this.repository.FindById(data);
      case "VIEW_PRODUCTS":
        return this.repository.FindSelectedProducts(data);
      default:
        return "Invalid Request";
    }
  }
}

module.exports = ProductService;
