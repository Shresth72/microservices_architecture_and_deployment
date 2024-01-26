-- Your SQL goes here
CREATE TABLE product (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    banner VARCHAR(255) NOT NULL,
    type_ VARCHAR(255) NOT NULL,
    unit INT NOT NULL,
    price FLOAT NOT NULL,
    available BOOLEAN NOT NULL,
    suplier VARCHAR(255) NOT NULL
);