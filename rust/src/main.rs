mod models;
use models::repository::*;

mod utils;
use utils::*;

mod services;
use services::*;

mod schema;
use self::schema::customer::dsl::*;

mod database;

fn main() {
    let h = 3;
}
