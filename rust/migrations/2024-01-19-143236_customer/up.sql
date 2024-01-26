-- Your SQL goes here
CREATE TABLE customer (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    address JSON[],
    cart JSON[],
    wishlist JSON[],
    orders JSON[]
);