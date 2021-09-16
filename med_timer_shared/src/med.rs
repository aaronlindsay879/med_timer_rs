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
