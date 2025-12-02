use sqlx::Error as SqlxError;

pub const UNIQUE_VIOLATION: &str = "23505";
pub const FOREIGN_KEY_VIOLATION: &str = "23503";
pub const NOT_NULL_VIOLATION: &str = "23502";

pub trait PgErrorExt {
    fn is_unique_violation(&self) -> bool;
    fn constraint_name(&self) -> Option<&str>;
    fn constraint_name_like(&self, name: &str) -> bool;
}

impl PgErrorExt for SqlxError {
    fn is_unique_violation(&self) -> bool {
        matches!(
            self,
            SqlxError::Database(db_error) if db_error.code().as_deref() == Some(UNIQUE_VIOLATION)
        )
    }

    fn constraint_name(&self) -> Option<&str> {
        match self {
            SqlxError::Database(e) => e.constraint(),
            _ => None
        }
    }

    fn constraint_name_like(&self, name: &str) -> bool {
        match self {
            SqlxError::Database(db_error) if db_error.message().contains(name) => {
                true
            },
            _ => false
        }
    }
}