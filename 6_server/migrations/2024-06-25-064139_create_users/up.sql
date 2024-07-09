-- Your SQL goes here
create table users
(
    id       uuid default gen_random_uuid() not null
        primary key,
    username text                           not null
        constraint users_name_key
            unique,
    email    text                           not null
        unique
);
