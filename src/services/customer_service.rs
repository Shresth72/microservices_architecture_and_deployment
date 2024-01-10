use crate::database::repository::customer::CustomerRepository::*;
use crate::index::{FormateData, GeneratePassword, ValidatePassword, GenerateSignature};
use crate::app_error::{APIError, BadRequestError};

pub struct CustomerService {
    pub repository: CustomerRepository,
}

pub struct UserInputs {
    pub email: String,
    pub password: String,
}

impl CustomerService {
    pub fn new(repository: CustomerRepository) -> CustomerService {
        CustomerService { repository }
    }

    pub async fn sign_in(user_inputs: UserInputs) {
        let email = user_inputs.email;
        let password = user_inputs.password;

        let existing_customer = FindCustomer(email).await
    }
}

