-- migrations/20241126000004_create_book_authors.sql
CREATE TABLE IF NOT EXISTS book_authors (
    id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
    author_order SMALLINT DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(book_id, author_id)
);

CREATE INDEX idx_book_authors_book_id ON book_authors (book_id);
CREATE INDEX idx_book_authors_author_id ON book_authors (author_id);