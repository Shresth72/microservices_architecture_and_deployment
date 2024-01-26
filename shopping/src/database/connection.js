const mongoose = require('mongoose');
const { MONGODB_URI } = require('../config');
const dotEnv = require("dotenv");

module.exports = async() => {
    dotEnv.config();

    try {
        await mongoose.connect(process.env.MONGODB_URI);
        console.log('Db Connected');
        
    } catch (error) {
        console.log('Error ============')
        console.log(error);
        process.exit(1);
    }
 
};
