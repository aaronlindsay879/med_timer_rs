use crate::{query, routes::DefaultQuery};
use futures::StreamExt;
use med_timer_shared::entry::Entry;
use paperclip::actix::{
    api_v2_operation, get,
    web::{self, Json, Path},
    Apiv2Schema,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

/// Fetches the most recent 100 entries for all medications.
#[get("/")]
#[api_v2_operation]
async fn get_all_entries(
    db_pool: web::Data<SqlitePool>,
    queries: web::Query<DefaultQuery>,
) -> Json<Vec<Entry>> {
    log::trace!("searching database for all entries");

    query!(
        pool: db_pool,
        queries: queries,
        out: Vec<Entry>,
        "SELECT * FROM entry ORDER BY datetime(time) DESC"
    )
}

/// Fetches the most recent entry for a given entry UUID.
#[get("/by-entry-uuid/{entry_uuid}/")]
#[api_v2_operation]
async fn get_entries_from_entry(
    Path(entry_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> Json<Option<Entry>> {
    log::trace!("searching database for entry with uuid: `{}`", entry_uuid);

    query!(
        pool: db_pool,
        out: Option<Entry>,
        "SELECT * FROM entry WHERE uuid LIKE ? ORDER BY datetime(time) DESC",
        entry_uuid
    )
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

    query!(
        pool: db_pool,
        queries: queries,
        out: Vec<Entry>,
        "SELECT * FROM entry WHERE medication_uuid LIKE ? ORDER BY datetime(time) DESC",
        medication_uuid
    )
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

    query!(
        pool: db_pool,
        queries: queries,
        out: Vec<CombinedEntryMed>,
        "SELECT
            entry.uuid AS entry_uuid,
            entry.amount AS entry_amount,
            entry.time AS entry_time,
            entry.medication_uuid,
            medication.name AS medication_name
        FROM entry
        INNER JOIN medication
            ON medication.uuid = entry.medication_uuid
            AND medication.name = ?
        ORDER BY datetime(time) DESC",
        medication_name
    )
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
