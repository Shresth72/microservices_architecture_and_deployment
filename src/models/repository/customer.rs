use std::{ops::Add, error::Error};

use crate::{product::ProductRepository, services::customer_service::CustomerService, utils::app_error::{AppError, StatusCode}};
use aws_sdk_dynamodb::{Client, types::{AttributeValue, error::{ReplicaNotFoundException, ResourceNotFoundException}}};
use nanoid::nanoid;

pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

pub struct CartItem {
    pub product: ProductRepository,
    pub unit: u32,
}

pub struct CustomerRepository {
    pub id: String,
    pub email: String,    //
    pub password: String, //
    pub phone: String,    //
    pub address: Vec<Address>,
    pub cart: Vec<CartItem>,
    pub wishlist: Vec<ProductRepository>,
    pub orders: Vec<String>,
}

pub struct CustomerInput {
    pub email: String,
    pub password: String,
    pub phone: String,
}

impl CustomerRepository {
    pub async fn new(input: CustomerInput) -> CustomerRepository {
        let id = nanoid!(10);
        let email = input.email;
        let password = input.password;
        let phone = input.phone;

        let customer = CustomerRepository {
            id,
            email,
            password,
            phone,
            address: Vec::new(),
            cart: Vec::new(),
            wishlist: Vec::new(),
            orders: Vec::new(),
        };

        customer
    }

    pub async fn create_address(input: Address) -> Address {
        input
    }

    // pub async fn find_customer(client: &Client, table_name: &String, auth: bool, email: String) -> Result<Self, aws_sdk_dynamodb::Error> {
    //     // TODO: Find customer from database
    //     let mut request = client
    //         .query()
    //         .table_name(table_name)
    //         .key_condition_expression("#email = :email")
    //         .expression_attribute_names("#email", "email")
    //         .expression_attribute_values(":email", AttributeValue::S(email))
    //         .send()
    //         .await?;

    //     if let Some(items) = request.items {
    //         let customer = items[0].clone();
    //         let customer = CustomerRepository::new(CustomerInput {
    //             email: customer.get("email").unwrap().as_s().unwrap().to_string(),
    //             password: customer.get("password").unwrap().as_s().unwrap().to_string(),
    //             phone: customer.get("phone").unwrap().as_s().unwrap().to_string(),
    //         }).await;

    //         Ok(customer)
    //     } else if auth == false {
    //         // return empty customer
    //         let customer = CustomerRepository::new(CustomerInput {
    //             email: "".to_string(),
    //             password: "".to_string(),
    //             phone: "".to_string(),
    //         }).await;
    //         Ok(customer)
    //     } else {
            
    //     }
    // }

    pub async fn find_customer(
        client: &Client,
        table_name: &str, // Use &str instead of &String for table_name
        auth: bool,
        email: String,
    ) -> Result<Self, Box<dyn Error>> {
        // TODO: Find customer from database
        let request = client
            .query()
            .table_name(table_name)
            .key_condition_expression("#email = :email")
            .expression_attribute_names("#email", "email")
            .expression_attribute_values(":email", AttributeValue::S(email))
            .send()
            .await?;
    
        if let Some(items) = request.items {
            if items.is_empty() {
                // No customer found
                if auth {
                    return Err(Box::from("Customer not found"))
                }
            } else {
                // Customer found
                let customer = items[0].clone();
                let customer = CustomerRepository::new(CustomerInput {
                    email: customer.get("email").unwrap().as_s().unwrap().to_string(),
                    password: customer.get("password").unwrap().as_s().unwrap().to_string(),
                    phone: customer.get("phone").unwrap().as_s().unwrap().to_string(),
                })
                .await;
    
                return Ok(customer);
            }
        }
    
        if auth {
            // Customer not found, and auth is true, return an error
            return Err(Box::from("Customer not found"))
        }
    
        // Return empty customer
        let customer = CustomerRepository::new(CustomerInput {
            email: "".to_string(),
            password: "".to_string(),
            phone: "".to_string(),
        })
        .await;
        Ok(customer)
    }

    pub async fn find_customer_by_id(self, id: String) {
        // TODO: Find customer from database
    }

    pub async fn find_wishlist(&mut self, product: ProductRepository) -> &mut Self {
        self
    }

    pub async fn add_wishlist_item(
        &mut self,
        customerId: String,
        product: ProductRepository,
    ) -> &mut Self {
        self
    }

    pub async fn RemoveWishlistItem(
        &mut self,
        customerId: String,
        product: ProductRepository,
    ) -> &mut Self {
        self
    }

    pub async fn FindCart(self, product: ProductRepository) -> Self {
        self
    }

    pub async fn AddCartItem(
        &mut self,
        customerId: String,
        product: ProductRepository,
    ) -> &mut Self {
        self
    }

    pub async fn RemoveCartItem(
        &mut self,
        customerId: String,
        product: ProductRepository,
    ) -> &mut Self {
        self
    }

    pub async fn FindOrders(self, customerId: String) -> Self {
        self
    }

    pub async fn AddOrderToProfile(&mut self, customerId: String, orderId: String) -> &mut Self {
        self
    }

    pub async fn RemoveOrderFromProfile(
        &mut self,
        customerId: String,
        orderId: String,
    ) -> &mut Self {
        self
    }
}
