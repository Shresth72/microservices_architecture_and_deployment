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
    pub fn new() -> Self {
        OrderRepository {
            order_id: String::new(),
            customer_id: String::new(),
            amount: 0.0,
            status: String::new(),
            txn_id: String::new(),
            items: Vec::new(),
        }
    }

    async fn Orders(&self, customer_id: String) {
        // TODO: Find orders from database
    }

    async fn OrderById(&self, customer_id: String, txn_id: String) {
        // TODO: Find order by id from database
    }
}
