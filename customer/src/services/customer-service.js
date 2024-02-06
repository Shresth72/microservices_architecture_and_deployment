const { CustomerRepository } = require("../database");
const {
  GeneratePassword,
  GenerateSalt,
  GenerateSignature,
  ValidatePassword
} = require("../utils");
const {
  APIError,
  NotFoundError,
  ValidationError
} = require("../utils/errors/app-errors");

class CustomerService {
  constructor() {
    this.repository = new CustomerRepository();
  }

  async SignIn(userInputs) {
    const { email, password } = userInputs;

    try {
      const existingCustomer = await this.repository.FindCustomer({ email });

      if (!existingCustomer)
        throw new NotFoundError("user not found with provided email");

      const validPassword = await ValidatePassword(
        password,
        existingCustomer.password,
        existingCustomer.salt
      );
      if (!validPassword) throw new ValidationError("password does not match");

      const token = await GenerateSignature({
        email: existingCustomer.email,
        _id: existingCustomer._id
      });

      return { id: existingCustomer._id, token };
    } catch (err) {
      throw new APIError("error in signing in", err);
    }
  }

  async SignUp(userInputs) {
    const { email, password, phone } = userInputs;

    try {
      // create salt
      let salt = await GenerateSalt();

      let userPassword = await GeneratePassword(password, salt);

      const existingCustomer = await this.repository.CreateCustomer({
        email,
        password: userPassword,
        phone,
        salt
      });

      const token = await GenerateSignature({
        email: email,
        _id: existingCustomer._id
      });

      return { id: existingCustomer._id, token };
    } catch (err) {
      throw new APIError("error in creating user", err);
    }
  }

  async AddNewAddress(_id, userInputs) {
    const { street, postalCode, city, country } = userInputs;

    try {
      return this.repository.CreateAddress({
        _id,
        street,
        postalCode,
        city,
        country
      });
    } catch (err) {
      throw new APIError("error in creating address", err);
    }
  }

  async GetProfile(id) {
    try {
      return this.repository.FindCustomerById({ id });
    } catch (err) {
      throw new APIError("profile not found", err);
    }
  }

  async DeleteProfile(id) {
    try {
      const data = await this.repository.DeleteCustomerById({ id });
      const payload = {
        event: "DELETE_PROFILE",
        data: { userId: id }
      };
      return {
        data,
        payload
      };
    } catch (err) {
      throw new APIError("error in deleting profile", err);
    }
  }
}

module.exports = CustomerService;
