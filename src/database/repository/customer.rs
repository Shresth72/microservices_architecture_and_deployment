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
    pub salt: String,     //
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
    pub salt: String,
}

impl CustomerRepository {
    pub async fn CreateCustomer(&mut self, input: CustomerInput) -> &mut Self {
        self.id = nanoid!();
        self.email = input.email;
        self.password = input.password;
        self.phone = input.phone;
        self.salt = input.salt;

        // TODO: Push to database

        self
    }

    pub async fn CreateAddress(&mut self, input: Address) -> &mut Self {
        self.address.push(input);

        // TODO: Push to database

        self
    }

    pub async fn FindCustomer(self, email: String) {
        // TODO: Find customer from database
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
