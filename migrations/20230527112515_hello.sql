-- Add migration script here
CREATE TABLE IF NOT EXISTS courses (
    id serial primary key,
    name varchar(40) not null unique,
    department varchar(30) not null
);
