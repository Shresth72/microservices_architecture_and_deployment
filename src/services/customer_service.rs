use crate::customer::{CustomerRepository, Address};
use crate::database::repository::customer::CustomerInput;
use crate::index::{FormateData, GeneratePassword, ValidatePassword, GenerateSignature};
use crate::app_error::{APIError, BadRequestError};
use crate::utils::app_error::{AppError, StatusCode};
use crate::utils::index::TokenClaims;

pub struct CustomerService {
    pub repository: CustomerRepository,
}

pub struct SignInUserInputs {
    pub email: String,
    pub password: String,
}

pub struct SignUpUserInputs {
    pub email: String,
    pub password: String,
    pub phone: String,
}

impl CustomerService {
    pub fn new(repository: CustomerRepository) -> CustomerService {
        CustomerService { repository }
    }

    pub async fn sign_in(user_inputs: SignInUserInputs) -> Result<String, AppError> {
        let email = user_inputs.email;
        let entered_password = user_inputs.password;

        let customer = CustomerRepository::FindCustomer(email).await;

        if customer.email == "".to_string() {
            return Err(AppError::new(
                "Email not found".to_string(),
                StatusCode::NotFound,
                "Not Found".to_string(),
            ));
        }

        let is_valid = ValidatePassword(entered_password, customer.password).await;
        if is_valid {
            let payload: TokenClaims = TokenClaims {
                email: customer.email,
                id: customer.id,
            };

            let token: String = GenerateSignature(payload).await.unwrap();
            // return FormateData(token);
            return Ok(token);
        }
        
        Err(AppError::new(
            "Password is incorrect".to_string(),
            StatusCode::Unauthorized,
            "Unauthorized".to_string(),
        ))
    }

    pub async fn sign_up(user_inputs: SignUpUserInputs) -> Result<String, String> {
        let email = user_inputs.email;
        let password = user_inputs.password;
        let phone = user_inputs.phone;

        let user_password = GeneratePassword(password).await;

        let customer_input: CustomerInput = CustomerInput {
            email: email.clone(),
            password: user_password,
            phone: phone.clone(),
        };
        let customer = CustomerRepository::CreateCustomer(customer_input).await;

        let token = GenerateSignature(TokenClaims {
            email: customer.email,
            id: customer.id,
        }).await.unwrap();

        Ok(token)
    }

    pub async fn new_address(user_inputs: Address) -> Result<Address, String> {
        let address_result = CustomerRepository::CreateAddress(user_inputs).await;

        Ok(address_result)
    }
}

