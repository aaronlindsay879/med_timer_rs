use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Stores information about a single medication.
#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, FromRow)]
pub struct Med {
    name: String,
    uuid: String,
}

impl Med {
    /// Creates a new medication with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4().to_string(),
        }
    }
}

// impl<'r> sqlx::FromRow<'r, SqliteRow> for Med {
//     fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
//         let name = row.get("name");

//         // if invalid uuid, map to closest possible sqlx error and return
//         let uuid = row.get("uuid");

//         log::trace!("constructed medication from database: `{}`", uuid);
//         Ok(Self { name, uuid })
//     }
// }
