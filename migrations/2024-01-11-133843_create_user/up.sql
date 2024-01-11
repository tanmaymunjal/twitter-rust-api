-- Your SQL goes here
CREATE TABLE users (
    user_email VARCHAR(320) PRIMARY KEY,
    user_name VARCHAR(100) NOT NULL,
    user_profile_pic bytea,
    user_twitter_banner bytea,
    created_at  timestamptz NOT NULL DEFAULT NOW(),
    updated_at  timestamptz DEFAULT NULL
)