use crate::{customer::CustomerRepository, product::ProductRepository};

pub struct OrderItem {
    pub product: ProductRepository,
    pub unit: u32,
}

pub struct OrderRepository {
    order_id: String,
    customer_id: String,
    amount: f32,
    status: String,
    txn_id: String,
    items: Vec<OrderItem>,
}

impl OrderRepository {
    pub async fn orders(self, customer_id: String) -> Vec<OrderRepository> {
        let orders = vec![
            OrderRepository {
                order_id: "".to_string(),
                customer_id: customer_id.clone(),
                amount: 0.0,
                status: "".to_string(),
                txn_id: "".to_string(),
                items: Vec::new(),
            },
            OrderRepository {
                order_id: "".to_string(),
                customer_id: customer_id,
                amount: 0.0,
                status: "".to_string(),
                txn_id: "".to_string(),
                items: Vec::new(),
            },
        ];
        
        orders
    }

    pub async fn create_new_order(self, customer_id: String, txn_id: String) -> Self {
        OrderRepository {
            order_id: "".to_string(),
            customer_id,
            amount: 0.0,
            status: "".to_string(),
            txn_id,
            items: Vec::new(),
        }
    }
}
