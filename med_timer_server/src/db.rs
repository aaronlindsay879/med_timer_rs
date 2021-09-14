use med_timer_shared::med::Med;
use sqlx::SqlitePool;

/// Finds one or more medication entries from the database.
/// If uuid is `None`, finds all medications. If Some(string), then finds all medications with matching UUIDs (hopefully just one).
pub(crate) async fn find_med(
    uuid: Option<String>,
    pool: &SqlitePool,
) -> Result<Vec<Med>, sqlx::Error> {
    let query = match uuid {
        Some(uuid) => {
            log::trace!("searching database for medication: `{}`", uuid);
            sqlx::query_as("SELECT * FROM medication WHERE uuid LIKE ?").bind(uuid)
        }
        None => {
            log::trace!("searching databse for all medications");
            sqlx::query_as("SELECT * FROM medication")
        }
    };

    query.fetch_all(pool).await
}
