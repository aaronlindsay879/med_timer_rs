use actix_web::{
    get,
    web::{self, Path},
    HttpResponse, Responder,
};
use med_timer_shared::entry::Entry;
use sqlx::SqlitePool;

/// Generates a json response from the given medication + entry uuids and database pool.
/// If results are found, simply return those results.
/// Ohterwise serve empty json.
async fn generate_response(
    entry_uuid: Option<String>,
    medication_uuid: Option<String>,
    db_pool: &SqlitePool,
) -> HttpResponse {
    log::trace!(
        "searching database for medication uuid: `{}`; entry uuid: `{}`",
        medication_uuid.clone().unwrap_or_else(|| "*".to_string()),
        entry_uuid.clone().unwrap_or_else(|| "*".to_string())
    );

    let db_response: Result<Vec<Entry>, sqlx::Error> =
        sqlx::query_as("SELECT * FROM entry WHERE uuid LIKE ? AND medication_uuid LIKE ?")
            .bind(entry_uuid.unwrap_or_else(|| "%".to_string()))
            .bind(medication_uuid.unwrap_or_else(|| "%".to_string()))
            .fetch_all(db_pool)
            .await;

    match db_response {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::Ok().json([0; 0]),
    }
}

#[get("/")]
async fn get_all_entries(db_pool: web::Data<SqlitePool>) -> impl Responder {
    generate_response(None, None, &db_pool).await
}

#[get("/{entry_uuid}/")]
async fn get_entries_from_entry(
    Path(entry_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    generate_response(Some(entry_uuid), None, &db_pool).await
}

#[get("/by-medication/{medication_uuid}/")]
async fn get_entries_from_medication(
    Path(medication_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    generate_response(None, Some(medication_uuid), &db_pool).await
}

/// Adds all entry services to config
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("entry")
            .service(get_all_entries)
            .service(get_entries_from_entry)
            .service(get_entries_from_medication),
    );
}
