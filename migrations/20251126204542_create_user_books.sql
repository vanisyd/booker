-- migrations/20241126000006_create_user_books.sql
CREATE TYPE reading_status AS ENUM ('want_to_read', 'reading', 'completed', 'abandoned');

CREATE TABLE IF NOT EXISTS user_books (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    isbn_id INTEGER REFERENCES isbns(id) ON DELETE SET NULL,
    status reading_status NOT NULL DEFAULT 'want_to_read',
    progress SMALLINT CHECK (progress >= 0 AND progress <= 100) DEFAULT 0,
    current_page INTEGER DEFAULT 0,
    rating SMALLINT CHECK (rating >= 1 AND rating <= 5),
    notes TEXT,
    started_at TIMESTAMP WITH TIME ZONE,
    finished_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, book_id)
);

CREATE INDEX idx_user_books_user_id ON user_books (user_id);
CREATE INDEX idx_user_books_book_id ON user_books (book_id);
CREATE INDEX idx_user_books_status ON user_books (status);