use crate::order::OrderRepository;
use crate::index::FormateData;
use crate::utils::app_error::APIError;

pub struct ShoppingService {
    pub repository: OrderRepository,
}

impl ShoppingService {
    pub async fn place_order(self, customer_id: String, txn_id: String) -> Result<OrderRepository, APIError> {
        let order = self.repository.create_new_order(customer_id, txn_id).await;

        match FormateData(order) {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::new("Failed to formate data".to_string())),
        }
    }

    pub async fn get_orders(self, customer_id: String) -> Result<Vec<OrderRepository>, APIError> {
        let orders = self.repository.orders(customer_id).await;

        match FormateData(orders) {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::new("Failed to formate data".to_string())),
        }
    }
}
