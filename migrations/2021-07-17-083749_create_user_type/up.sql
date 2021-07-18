-- Your SQL goes here
CREATE TABLE user_role (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    auth_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT 't'
);

SELECT diesel_manage_updated_at('user_role');

INSERT INTO user_role (auth_type) VALUES('SUPER_ADMIN');

INSERT INTO user_role (auth_type) VALUES('ADMIN');

INSERT INTO user_role (auth_type) VALUES('STAFF');

-- INSERT INTO user_role (auth_type) VALUES('SUPER_STAFF');

-- INSERT INTO user_role (auth_type) VALUES('AUDITOR');

-- INSERT INTO user_role (auth_type) VALUES('E_USER');

INSERT INTO user_role (auth_type) VALUES('USER');
