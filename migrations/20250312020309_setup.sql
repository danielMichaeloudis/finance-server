create table
    if not exists users (
        user_uuid uuid primary key default gen_random_uuid (),
        username varchar(255) not null unique,
        email varchar(255) not null unique,
        password_hash varchar(255) not null,
        created_at timestamp not null default now (),
        encryption_key bytea unique not null
    );

create table
    if not exists families (
        family_uuid uuid primary key default gen_random_uuid (),
        join_code bytea unique not null,
        join_code_prefix varchar(255) unique not null,
        encryption_key bytea unique not null
    );

create table
    if not exists link_users_families (
        user_uuid uuid NOT NULL,
        family_uuid uuid NOT NULL,
        CONSTRAINT fk_user FOREIGN KEY (user_uuid) REFERENCES users (user_uuid) ON DELETE CASCADE,
        CONSTRAINT fk_family FOREIGN KEY (family_uuid) REFERENCES families (family_uuid) ON DELETE CASCADE,
        primary key (user_uuid, family_uuid)
    );

create table
    if not exists family_data (
        uuid uuid primary key default gen_random_uuid (),
        family_uuid uuid not null,
        encrypted_data bytea not null,
        data_time timestamp not null default now (),
        CONSTRAINT fk_family FOREIGN KEY (family_uuid) REFERENCES families (family_uuid) ON DELETE CASCADE
    );

create table
    if not exists user_data (
        uuid uuid primary key default gen_random_uuid (),
        user_uuid uuid not null,
        encrypted_data bytea not null,
        data_time timestamp not null default now (),
        CONSTRAINT fk_user FOREIGN KEY (user_uuid) REFERENCES users (user_uuid) ON DELETE CASCADE
    );