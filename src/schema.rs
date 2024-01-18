// @generated automatically by Diesel CLI.

diesel::table! {
    customer (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        address -> Nullable<Array<Nullable<Json>>>,
        cart -> Nullable<Array<Nullable<Json>>>,
        wishlist -> Nullable<Array<Nullable<Json>>>,
        orders -> Nullable<Array<Nullable<Json>>>,
    }
}

diesel::table! {
    product (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        banner -> Varchar,
        #[max_length = 255]
        type_ -> Varchar,
        unit -> Int4,
        price -> Float8,
        available -> Bool,
        #[max_length = 255]
        suplier -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customer,
    product,
);
