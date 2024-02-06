const { CustomerModel, AddressModel } = require("../models");
const { APIError, STATUS_CODES } = require("../../utils/errors/app-errors");

//Dealing with data base operations
class CustomerRepository {
  async CreateCustomer({ email, password, phone, salt }) {
    try {
      const customer = new CustomerModel({
        email,
        password,
        salt,
        phone,
        address: []
      });
      const customerResult = await customer.save();
      return customerResult;
    } catch (err) {
      throw new APIError("unable to create customer");
    }
  }

  async CreateAddress({ _id, street, postalCode, city, country }) {
    try {
      const profile = await CustomerModel.findById(_id);

      if (profile) {
        const newAddress = new AddressModel({
          street,
          postalCode,
          city,
          country
        });

        await newAddress.save();

        profile.address.push(newAddress);
      }

      return await profile.save();
    } catch (err) {
      throw new APIError("unable to create address");
    }
  }

  async FindCustomer({ email }) {
    try {
      const existingCustomer = await CustomerModel.findOne({ email: email });
      return existingCustomer;
    } catch (err) {
      throw new APIError("unable to find customer");
    }
  }

  async FindCustomerById({ id }) {
    try {
      const existingCustomer = await CustomerModel.findById(id).populate(
        "address"
      );
      return existingCustomer;
    } catch (err) {
      throw new APIError("unable to find customer");
    }
  }

  async DeleteCustomerById({ id }) {
    try {
      return CustomerModel.findByIdAndDelete(id);
    } catch (err) {
      throw new APIError("unable to delete customer");
    }
  }
}

module.exports = CustomerRepository;
