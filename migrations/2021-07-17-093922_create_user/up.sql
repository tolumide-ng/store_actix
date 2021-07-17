CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Your SQL goes here
CREATE TABLE user_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email VARCHAR NOT NULL,
    hash VARCHAR NOT NULL,
    user_type UUID NOT NULL,
    FOREIGN KEY (user_type) REFERENCES user_role(id) ON UPDATE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);


SELECT diesel_manage_updated_at('user_info');