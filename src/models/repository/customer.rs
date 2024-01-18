use std::error::Error;
use crate::product::ProductRepository;
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use diesel::{deserialize::Queryable, prelude::Insertable};
use nanoid::nanoid;
use maplit::hashmap;
use serde::Deserialize;
use diesel::prelude::*;
use super::super::super::schema::customer;

#[derive(Debug, Clone, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CartItem {
    pub product: ProductRepository,
    pub unit: u32,
}

#[derive(Queryable, Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = customer)]
pub struct CustomerRepository {
    pub id: String,
    pub email: String,
    pub password: String,
    pub phone: String,   
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

    pub async fn create_address(client: &Client, table_name: &str, id: &str, input: Address) -> Result<Address, Box<dyn Error>> {
        let address = Address {
            street: input.street,
            city: input.city,
            postal_code: input.postal_code,
            country: input.country,
        };

        // update the customer address
        let request = client.update_item()
            .table_name(table_name)
            .key("id", AttributeValue::S(id.to_string()))
            .update_expression("SET #address = list_append(#address, :address)")
            .expression_attribute_names("#address", "address")
            .expression_attribute_values(":address", AttributeValue::L(vec![
                AttributeValue::M(hashmap! {
                    "street".to_string() => AttributeValue::S(address.street.clone()),
                    "city".to_string() => AttributeValue::S(address.city.clone()),
                    "postal_code".to_string() => AttributeValue::S(address.postal_code.clone()),
                    "country".to_string() => AttributeValue::S(address.country.clone()),
                })
            ]))
            .send()
            .await?;

        if let Some(_) = request.attributes {
            return Ok(address)
        }

        Err(Box::from("Failed to create address"))
    }

    pub async fn create_customer(client: &Client, table_name: &str, input: CustomerInput) -> Result<Self, Box<dyn Error>> {
        let customer = CustomerRepository::new(input).await;

        let request = client
            .put_item()
            .table_name(table_name)
            .item("id", AttributeValue::S(customer.id.clone()))
            .item("email", AttributeValue::S(customer.email.clone()))
            .item("password", AttributeValue::S(customer.password.clone()))
            .item("phone", AttributeValue::S(customer.phone.clone()))
            .send()
            .await?;

        if let Some(_) = request.attributes {
            return Ok(customer)
        }

        Err(Box::from("Failed to create customer"))
    }

    pub async fn find_customer(
        client: &Client,
        table_name: &str,
        auth: bool,
        email: String,
    ) -> Result<Self, Box<dyn Error>> {
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

    pub async fn find_customer_by_id(client: &Client, table_name: &str, id: &str) -> Result<Self, Box<dyn Error>> {
        let request = client
            .query()
            .table_name(table_name)
            .key_condition_expression("#id = :id")
            .expression_attribute_names("#id", "id")
            .expression_attribute_values(":id", AttributeValue::S(id.to_string()))
            .send()
            .await?;

        if let Some(items) = request.items {
            if items.is_empty() {
                return Err(Box::from("Customer not found"))
            } else {
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

        // Return error
        Err(Box::from("Customer not found"))
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
