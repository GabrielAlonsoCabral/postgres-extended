CREATE TABLE if not exists companies (
    company_id serial PRIMARY KEY,
    company_name VARCHAR(255) NOT NULL,
    city_id INT,
    FOREIGN KEY (city_id) REFERENCES cities(city_id)
);