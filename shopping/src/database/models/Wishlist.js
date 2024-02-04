const mongoose = require("mongoose");

const Schema = mongoose.Schema;

const WishlistSchema = new Schema({
  customerId: String,
  products: [
    {
      _id: { type: String, required: true }
    }
  ]
});

module.exports = mongoose.model("wishlist", WishlistSchema);
