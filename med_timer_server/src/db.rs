use med_timer_shared::{entry::Entry, med::Med};
use sqlx::SqlitePool;

/// Finds one or more medications from the database.
/// If uuid is `None`, finds all medications. If Some(string), then finds all medications with matching UUIDs (hopefully just one).
pub(crate) async fn find_med(
    uuid: Option<String>,
    connection: &SqlitePool,
) -> Result<Vec<Med>, sqlx::Error> {
    log::trace!(
        "searching database for medication: `{}`",
        uuid.clone().unwrap_or_else(|| "*".to_string())
    );

    sqlx::query_as("SELECT * FROM medication WHERE uuid LIKE ?")
        .bind(uuid.unwrap_or_else(|| "%".to_string()))
        .fetch_all(connection)
        .await
}

/// Finds one or more medication entries from the database.
/// If uuid is `None`, finds all entries. If Some(string), then finds all entries with matching UUIDs (hopefully just one).
pub(crate) async fn find_entries(
    entry_uuid: Option<String>,
    medication_uuid: Option<String>,
    connection: &SqlitePool,
) -> Result<Vec<Entry>, sqlx::Error> {
    log::trace!(
        "searching database for medication uuid: `{}`; entry uuid: `{}`",
        medication_uuid.clone().unwrap_or_else(|| "*".to_string()),
        entry_uuid.clone().unwrap_or_else(|| "*".to_string())
    );

    sqlx::query_as("SELECT * FROM entry WHERE uuid LIKE ? AND medication_uuid LIKE ?")
        .bind(entry_uuid.unwrap_or_else(|| "%".to_string()))
        .bind(medication_uuid.unwrap_or_else(|| "%".to_string()))
        .fetch_all(connection)
        .await
}
