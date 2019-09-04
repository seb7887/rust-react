CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    bio VARCHAR(255),
    image VARCHAR(2048),
    token VARCHAR(255) NOT NULL
);