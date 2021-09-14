use serde::Serialize;
use sqlx::{sqlite::SqliteRow, Row};
use uuid::Uuid;

/// Stores information about a single medication.
#[derive(Debug, Serialize)]
pub struct Med {
    name: String,
    uuid: Uuid,
}

impl Med {
    /// Creates a new medication with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4(),
        }
    }
}

impl<'r> sqlx::FromRow<'r, SqliteRow> for Med {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let name = row.get("name");

        // if invalid uuid, map to closest possible sqlx error and return
        let uuid = Uuid::parse_str(row.get("uuid"))
            .map_err(|_| sqlx::Error::Decode("invalid UUID".into()))?;

        log::trace!("constructed medication from database: `{}`", uuid);
        Ok(Self { name, uuid })
    }
}
