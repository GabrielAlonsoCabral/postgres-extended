CREATE TABLE if not exists  users (
    user_id serial PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    company_id INT,
    FOREIGN KEY (company_id) REFERENCES companies(company_id)
);