-- migrations/20241126000003_create_authors.sql
CREATE TABLE IF NOT EXISTS authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    bio TEXT,
    photo_url VARCHAR(500),
    birth_year INTEGER,
    death_year INTEGER,
    external_keys JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_authors_external_keys ON authors USING GIN (external_keys);
CREATE INDEX idx_authors_name ON authors (name);