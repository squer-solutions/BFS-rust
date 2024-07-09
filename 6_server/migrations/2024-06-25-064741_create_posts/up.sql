-- Your SQL goes here
create table posts
(
    id        uuid default gen_random_uuid() not null
        constraint posts_pk
            primary key,
    title     text                           not null,
    body      text                           not null,
    published boolean                        not null
);
