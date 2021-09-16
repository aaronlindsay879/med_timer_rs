use chrono::{DateTime, TimeZone, Utc};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use uuid::Uuid;

/// Stores information about a single medicine entry.
#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, FromRow)]
pub struct Entry {
    amount: i64,
    time: String,
    medication_uuid: String,
    uuid: String,
}

impl Entry {
    /// Constructs a new entry with the given amount, at the given time (converting into UTC), for the given medicine id.
    pub fn new(amount: i64, time: DateTime<impl TimeZone>, medicine_id: Uuid) -> Self {
        let time = time.with_timezone(&Utc);

        Self {
            amount,
            time: time.to_rfc3339(),
            medication_uuid: medicine_id.to_string(),
            uuid: Uuid::new_v4().to_string(),
        }
    }

    /// Constructs a new entry with the given amount for the given medicine id at the current UTC time.
    pub fn new_now(amount: i64, medicine_id: Uuid) -> Self {
        Self {
            amount,
            time: Utc::now().to_rfc3339(),
            medication_uuid: medicine_id.to_string(),
            uuid: Uuid::new_v4().to_string(),
        }
    }
}
