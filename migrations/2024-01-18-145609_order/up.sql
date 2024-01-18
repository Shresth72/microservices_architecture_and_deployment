-- Your SQL goes here
CREATE TABLE "order" (
    order_id VARCHAR(255) NOT NULL PRIMARY KEY,
    customer_id VARCHAR(255) NOT NULL,
    amount FLOAT NOT NULL,
    status VARCHAR(255) NOT NULL,
    txn_id VARCHAR(255) NOT NULL,
    items JSON[],
    FOREIGN KEY (customer_id) REFERENCES customer (id)
);