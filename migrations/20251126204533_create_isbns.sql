-- migrations/20241126000005_create_isbns.sql
CREATE TABLE IF NOT EXISTS isbns (
    id SERIAL PRIMARY KEY,
    isbn VARCHAR(13) UNIQUE NOT NULL,
    book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    isbn_type VARCHAR(10) CHECK (isbn_type IN ('ISBN_10', 'ISBN_13')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_isbns_book_id ON isbns (book_id);
CREATE INDEX idx_isbns_isbn ON isbns (isbn);