const { ProductModel } = require("../models");
const { APIError } = require("../../utils/errors/app-errors");

//Dealing with data base operations
class ProductRepository {
  async CreateProduct({
    name,
    desc,
    type,
    unit,
    price,
    available,
    suplier,
    banner
  }) {
    try {
      const product = new ProductModel({
        name,
        desc,
        type,
        unit,
        price,
        available,
        suplier,
        banner
      });

      const productResult = await product.save();
      return productResult;
    } catch (err) {
      throw new APIError("unable to create product");
    }
  }

  async Products() {
    try {
      return await ProductModel.find();
    } catch (err) {
      throw new APIError("unable to fetch products");
    }
  }

  async FindById(id) {
    try {
      return await ProductModel.findById(id);
    } catch (err) {
      throw new APIError("unable to fetch product details");
    }
  }

  async FindByCategory(category) {
    try {
      const products = await ProductModel.find({ type: category });
      return products;
    } catch (err) {
      throw new APIError("unable to fetch products by category");
    }
  }

  async FindSelectedProducts(selectedIds) {
    try {
      const products = await ProductModel.find()
        .where("_id")
        .in(selectedIds.map((_id) => _id))
        .exec();
      return products;
    } catch (err) {
      throw new APIError("unable to fetch selected products");
    }
  }
}

module.exports = ProductRepository;
