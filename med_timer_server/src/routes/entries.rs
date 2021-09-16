use crate::{generate_response_functions, routes::DefaultQuery};
use med_timer_shared::entry::Entry;
use paperclip::actix::{
    api_v2_operation, get,
    web::{self, Json, Path},
    Apiv2Schema,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

generate_response_functions!(entry_response<Entry>, combined_response<CombinedEntryMed>);

/// Fetches the most recent 100 entries for all medications.
#[get("/")]
#[api_v2_operation]
async fn get_all_entries(
    db_pool: web::Data<SqlitePool>,
    queries: web::Query<DefaultQuery>,
) -> Json<Vec<Entry>> {
    log::trace!("searching database for all entries");

    let count = queries.count_or_default();
    let query = sqlx::query_as::<_, Entry>("SELECT * FROM entry ORDER BY datetime(time) DESC");

    Json(entry_response(query, count, &db_pool).await)
}

/// Fetches the most recent entry for a given entry UUID.
#[get("/by-entry-uuid/{entry_uuid}/")]
#[api_v2_operation]
async fn get_entries_from_entry(
    Path(entry_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> Json<Option<Entry>> {
    log::trace!("searching database for entry with uuid: `{}`", entry_uuid);

    let query =
        sqlx::query_as("SELECT * FROM entry WHERE uuid LIKE ? ORDER BY datetime(time) DESC")
            .bind(entry_uuid);

    Json(entry_response(query, 1, &db_pool).await.first().cloned())
}

/// Fetches the most recent 100 entries for a given medication UUID.
#[get("/by-med-uuid/{medication_uuid}/")]
#[api_v2_operation]
async fn get_entries_from_medication_uuid(
    Path(medication_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
    queries: web::Query<DefaultQuery>,
) -> Json<Vec<Entry>> {
    log::trace!(
        "searching database for entry with medication uuid: `{}`",
        medication_uuid
    );

    let count = queries.count_or_default();
    let query = sqlx::query_as(
        "SELECT * FROM entry WHERE medication_uuid LIKE ? ORDER BY datetime(time) DESC",
    )
    .bind(medication_uuid);

    Json(entry_response(query, count, &db_pool).await)
}

/// Represents combined information of entry + medication, removing need for second lookup in some situations.
#[derive(Apiv2Schema, FromRow, Serialize)]
struct CombinedEntryMed {
    entry_uuid: String,
    entry_amount: i64,
    entry_time: String,
    medication_uuid: String,
    medication_name: String,
}

/// Fetches the most recent 100 entries for a given medication name.
#[get("/by-med-name/{medication_name}/")]
#[api_v2_operation]
async fn get_entries_from_medication_name(
    Path(medication_name): Path<String>,
    db_pool: web::Data<SqlitePool>,
    queries: web::Query<DefaultQuery>,
) -> Json<Vec<CombinedEntryMed>> {
    log::trace!(
        "searching database for entry with medication name: `{}`",
        medication_name
    );

    let count = queries.count_or_default();
    let query = sqlx::query_as(
        "SELECT
            entry.uuid AS entry_uuid,
            amount AS entry_amount,
            time AS entry_time,
            medication_uuid,
            name AS medication_name
        FROM entry
        INNER JOIN medication
            ON medication.uuid = medication_uuid
            AND medication.name = ?
        ORDER BY datetime(time) DESC",
    )
    .bind(medication_name);

    Json(combined_response(query, count, &db_pool).await)
}

/// Adds all entry services to config
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::scope("entry")
    //         .service(web::resource("/").route(web::get().to(get_all_entries)))
    //         .service(
    //             web::resource("/by-entry-uuid/{entry_uuid}/")
    //                 .route(web::get().to(get_entries_from_entry)),
    //         )
    //         .service(
    //             web::resource("/by-med-uuid/{medication_uuid}/")
    //                 .route(web::get().to(get_entries_from_medication)),
    //         ),
    // );
    cfg.service(
        web::scope("entry")
            .service(get_all_entries)
            .service(get_entries_from_entry)
            .service(get_entries_from_medication_uuid)
            .service(get_entries_from_medication_name),
    );
}
