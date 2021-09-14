use chrono::{DateTime, TimeZone, Utc};
use serde::Serialize;
use uuid::Uuid;

/// Stores information about a single medicine entry.
#[derive(Debug, Serialize)]
pub struct Entry {
    amount: usize,
    time: DateTime<Utc>,
    medicine_id: Uuid,
}

impl Entry {
    /// Constructs a new entry with the given amount, at the given time (converting into UTC), for the given medicine id.
    pub fn new(amount: usize, time: DateTime<impl TimeZone>, medicine_id: Uuid) -> Self {
        let time = time.with_timezone(&Utc);

        Self {
            amount,
            time,
            medicine_id,
        }
    }

    /// Constructs a new entry with the given amount for the given medicine id at the current UTC time.
    pub fn new_now(amount: usize, medicine_id: Uuid) -> Self {
        Self {
            amount,
            time: Utc::now(),
            medicine_id,
        }
    }
}
