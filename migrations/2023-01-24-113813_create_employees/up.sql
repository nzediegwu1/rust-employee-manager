-- Your SQL goes here
CREATE TABLE employees (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR,
  department VARCHAR NOT NULL,
  salary INT NOT NULL,
  age SMALLINT NOT NULL,
  created_at TIMESTAMP NOT NULL
)