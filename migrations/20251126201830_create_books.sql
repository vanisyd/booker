-- migrations/20241126000002_create_books.sql
CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    cover_url VARCHAR(500),
    published_year INTEGER,
    page_count INTEGER,
    external_keys JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_books_external_keys ON books USING GIN (external_keys);
CREATE INDEX idx_books_title ON books (title);