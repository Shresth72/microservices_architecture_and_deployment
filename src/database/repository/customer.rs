use std::ops::Add;

use crate::product::ProductRepository;
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
    pub async fn CreateCustomer(input: CustomerInput) -> Self {
        CustomerRepository {
            id: nanoid!(),
            email: input.email,
            password: input.password,
            phone: input.phone,
            address: Vec::new(),
            cart: Vec::new(),
            wishlist: Vec::new(),
            orders: Vec::new(),
        }
    }

    pub async fn CreateAddress(input: Address) -> Address {
        input
    }

    pub async fn FindCustomer(email: String) -> Self {
        // TODO: Find customer from database
        CustomerRepository {
            id: nanoid!(),
            email,
            password: "".to_string(),
            phone: "".to_string(),
            address: Vec::new(),
            cart: Vec::new(),
            wishlist: Vec::new(),
            orders: Vec::new(),
        }
    }

    pub async fn FindCustomerById(self, id: String) {
        // TODO: Find customer from database
    }

    pub async fn FindWishlist(&mut self, product: ProductRepository) -> &mut Self {
        self
    }

    pub async fn AddWishlistItem(
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
