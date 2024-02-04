const mongoose = require('mongoose');

const Schema = mongoose.Schema;

const CartSchema = new Schema({
    customerId: String,
    items: [
        {   
            product: {
                _id: { type: String, required: true },
                name: { type: String },
                banner: { type: String },
                unit: { type: Number },
                price: { type: Number },
            },
            unit: { type: Number, require: true}
        }
    ]
},
{
    toJSON: {
        transform(doc, ret){
            delete ret.__v;
        }
    },
    timestamps: true
});

module.exports =  mongoose.model('cart', CartSchema);