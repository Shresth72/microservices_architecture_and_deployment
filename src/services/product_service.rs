use crate::product::{ProductRepository, ProductInput};
use crate::app_error::APIError;

pub struct ProductService {
    pub repository: ProductRepository,
    pub api_error: APIError,
}

impl ProductService {
    pub fn create_product(&mut self, input: ProductInput) -> Result<&mut ProductRepository, &APIError> {
        let product = self.repository.CreateProduct(input);
        Ok(product)
    }
}