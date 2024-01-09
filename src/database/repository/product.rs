use nanoid::nanoid;

pub struct ProductRepository {
    id: String,
    name: String,
    desc: String,
    banner: String,
    type_: String,
    unit: u32,
    price: f32,
    available: bool,
    suplier: String,
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
    pub fn CreateProduct(&mut self, input: ProductInput) -> &mut Self {
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
}
