use crate::customer::{CustomerRepository, Address};
use crate::models::repository::customer::CustomerInput;
use crate::index::{FormateData, GeneratePassword, ValidatePassword, GenerateSignature};
use crate::app_error::{APIError, BadRequestError};
use crate::utils::app_error::{AppError, StatusCode};
use crate::utils::index::TokenClaims;

use aws_sdk_dynamodb::{Client, config};
use aws_config::SdkConfig;
use std::collections::HashMap;

pub struct CustomerService {
    pub client: Client,
    pub table_name: String,
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
    pub async fn init(config: SdkConfig, table_name: String) -> CustomerService {
        let client = Client::new(&config);
        CustomerService {
            client,
            table_name,
        }
    }

    pub async fn sign_in(&self, user_inputs: SignInUserInputs) -> Result<String, AppError> {
        let email = user_inputs.email;
        let entered_password = user_inputs.password;

        let customer = CustomerRepository::find_customer(&self.client, &self.table_name, true, email).await;

        if customer.is_err() {
            return Err(AppError::new(
                "Email not found".to_string(),
                StatusCode::NotFound,
                "Not Found".to_string(),
            ));
        }

        let customer = customer.ok().unwrap();
        let email = customer.email;
        let hashed_password = customer.password;
        let id = customer.id;

        let is_valid = ValidatePassword(entered_password, hashed_password).await;
        if is_valid {
            let payload: TokenClaims = TokenClaims {
                email,
                id,
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
        let customer = CustomerRepository::create_customer(customer_input).await;

        let token = GenerateSignature(TokenClaims {
            email: customer.email,
            id: customer.id,
        }).await.unwrap();

        match FormateData(token) {
            Ok(token_data) => Ok(token_data),
            Err(_) => Err("Failed to formate data".to_string()),
        }
    }

    pub async fn new_address(user_inputs: Address) -> Result<Address, String> {
        let address_result = CustomerRepository::create_address(user_inputs).await;

        match FormateData(address_result) {
            Ok(address_data) => Ok(address_data),
            Err(_) => Err("Failed to formate data".to_string()),
        }
    }
}

