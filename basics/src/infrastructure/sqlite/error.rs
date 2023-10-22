use crate::domain::repositories::Error;

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Simple(value.to_string())
    }
}