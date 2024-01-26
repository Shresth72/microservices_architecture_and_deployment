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
    order (order_id) {
        #[max_length = 255]
        order_id -> Varchar,
        #[max_length = 255]
        customer_id -> Varchar,
        amount -> Float8,
        #[max_length = 255]
        status -> Varchar,
        #[max_length = 255]
        txn_id -> Varchar,
        items -> Nullable<Array<Nullable<Json>>>,
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

diesel::joinable!(order -> customer (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer,
    order,
    product,
);
