use nanoid::nanoid;

pub struct ProductRepository {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub banner: String,
    pub type_: String,
    pub unit: u32,
    pub price: f32,
    pub available: bool,
    pub suplier: String,
}

pub struct ProductInput {
    pub name: String,
    pub desc: String,
    pub banner: String,
    pub type_: String,
    pub unit: u32,
    pub price: f32,
    pub available: bool,
    pub suplier: String,
}

impl ProductRepository {
    pub fn create_product(&mut self, input: ProductInput) -> &mut Self {
        self.id = nanoid!();
        self.name = input.name;
        self.desc = input.desc;
        self.banner = input.banner;
        self.type_ = input.type_;
        self.unit = input.unit;
        self.price = input.price;
        self.available = input.available;
        self.suplier = input.suplier;

        // TODO: Push to database

        self
    }

    pub async fn get_products(self) -> Vec<ProductRepository> {
        let products = vec![
            ProductRepository {
                id: nanoid!(),
                name: "Product 1".to_string(),
                desc: "Product 1 description".to_string(),
                banner: "Product 1 banner".to_string(),
                type_: "Product 1 type".to_string(),
                unit: 1,
                price: 1.0,
                available: true,
                suplier: "Product 1 suplier".to_string(),
            },
            ProductRepository {
                id: nanoid!(),
                name: "Product 2".to_string(),
                desc: "Product 2 description".to_string(),
                banner: "Product 2 banner".to_string(),
                type_: "Product 2 type".to_string(),
                unit: 2,
                price: 2.0,
                available: true,
                suplier: "Product 2 suplier".to_string(),
            },
            ProductRepository {
                id: nanoid!(),
                name: "Product 3".to_string(),
                desc: "Product 3 description".to_string(),
                banner: "Product 3 banner".to_string(),
                type_: "Product 3 type".to_string(),
                unit: 3,
                price: 3.0,
                available: true,
                suplier: "Product 3 suplier".to_string(),
            },
        ];

        products
    }

    pub async fn find_by_id(self, id: String) -> ProductRepository {
        let product = ProductRepository {
            id: nanoid!(),
            name: "Product 1".to_string(),
            desc: "Product 1 description".to_string(),
            banner: "Product 1 banner".to_string(),
            type_: "Product 1 type".to_string(),
            unit: 1,
            price: 1.0,
            available: true,
            suplier: "Product 1 suplier".to_string(),
        };

        product
    }

    pub async fn find_by_category(self, category: String) -> Vec<ProductRepository> {
        let products = vec![
            ProductRepository {
                id: nanoid!(),
                name: "Product 1".to_string(),
                desc: "Product 1 description".to_string(),
                banner: "Product 1 banner".to_string(),
                type_: "Product 1 type".to_string(),
                unit: 1,
                price: 1.0,
                available: true,
                suplier: "Product 1 suplier".to_string(),
            },
            ProductRepository {
                id: nanoid!(),
                name: "Product 2".to_string(),
                desc: "Product 2 description".to_string(),
                banner: "Product 2 banner".to_string(),
                type_: "Product 2 type".to_string(),
                unit: 2,
                price: 2.0,
                available: true,
                suplier: "Product 2 suplier".to_string(),
            },
            ProductRepository {
                id: nanoid!(),
                name: "Product 3".to_string(),
                desc: "Product 3 description".to_string(),
                banner: "Product 3 banner".to_string(),
                type_: "Product 3 type".to_string(),
                unit: 3,
                price: 3.0,
                available: true,
                suplier: "Product 3 suplier".to_string(),
            },
        ];

        products
    }

    pub async fn find_selected_products(self, ids: Vec<String>) -> Vec<ProductRepository> {
        let products = vec![
            ProductRepository {
                id: nanoid!(),
                name: "Product 1".to_string(),
                desc: "Product 1 description".to_string(),
                banner: "Product 1 banner".to_string(),
                type_: "Product 1 type".to_string(),
                unit: 1,
                price: 1.0,
                available: true,
                suplier: "Product 1 suplier".to_string(),
            },
            ProductRepository {
                id: nanoid!(),
                name: "Product 2".to_string(),
                desc: "Product 2 description".to_string(),
                banner: "Product 2 banner".to_string(),
                type_: "Product 2 type".to_string(),
                unit: 2,
                price: 2.0,
                available: true,
                suplier: "Product 2 suplier".to_string(),
            },
            ProductRepository {
                id: nanoid!(),
                name: "Product 3".to_string(),
                desc: "Product 3 description".to_string(),
                banner: "Product 3 banner".to_string(),
                type_: "Product 3 type".to_string(),
                unit: 3,
                price: 3.0,
                available: true,
                suplier: "Product 3 suplier".to_string(),
            },
        ];

        products
    }
}
