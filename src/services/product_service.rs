use std::collections::HashMap;
use rocket::form::Form;
use rocket::futures::FutureExt;

use crate::database::repository::product;
use crate::product::{ProductRepository, ProductInput};
use crate::app_error::APIError;
use crate::utils::index::FormateData;

pub struct ProductService {
    pub repository: ProductRepository,
    pub api_error: APIError,
}

pub struct ProductsOutput {
    pub products: Vec<ProductRepository>,
    pub categories: Vec<String>,
}

impl ProductService {
    pub fn create_product(&mut self, input: ProductInput) -> Result<&mut ProductRepository, &APIError> {
        let product = self.repository.create_product(input);
        Ok(product)
    }

    pub async fn get_product(self, id: String) -> Result<ProductsOutput, APIError> {
        let products = self.repository.get_products().await;
        if products.len() == 0 {
            return Err(APIError::new("No products found".to_string()));
        }

        let mut categories: HashMap<String, String> = HashMap::new();
        
        let _ = products.iter().map(|product| {
            categories.entry(product.type_.clone()).or_insert(product.type_.clone());
        });

        let mut categories = categories.keys().cloned().collect::<Vec<String>>();
        categories.sort();

        let output: ProductsOutput = ProductsOutput {
            products,
            categories,
        };

        match FormateData(output) {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::new("Failed to formate data".to_string())),
        }
    }

    pub async fn get_product_description(self, product_id: String) -> Result<ProductRepository, APIError> {
        let product = self.repository.find_by_id(product_id).await;

        match FormateData(product) {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::new("Failed to formate data".to_string())),
        }
    }

    pub async fn get_product_by_category(self, category: String) -> Result<Vec<ProductRepository>, APIError> {
        let products = self.repository.find_by_category(category).await;

        match FormateData(products) {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::new("Failed to formate data".to_string())),
        }
    }

    pub async fn get_selected_products(self, product_ids: Vec<String>) -> Result<Vec<ProductRepository>, APIError> {
        let products = self.repository.find_selected_products(product_ids).await;

        match FormateData(products) {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::new("Failed to formate data".to_string())),
        }
    }
}