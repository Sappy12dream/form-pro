-- migrations/{timestamp}_create_users/up.sql
CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    first_name VARCHAR NOT NULL,    
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
