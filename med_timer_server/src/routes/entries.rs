use actix_web::{
    get,
    web::{self, Path},
    HttpResponse, Responder,
};
use med_timer_shared::entry::Entry;
use sqlx::SqlitePool;

use super::Query;

/// Generates a json response from the given query and database pool.
/// If results are found, simply return those results.
/// Otherwise serve empty json.
async fn generate_response(query: Query<'_, Entry>, db_pool: &SqlitePool) -> HttpResponse {
    match query.fetch_all(db_pool).await {
        Ok(meds) => HttpResponse::Ok().json(meds),
        Err(_) => HttpResponse::Ok().json([0; 0]),
    }
}

#[get("/")]
async fn get_all_entries(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let query = sqlx::query_as("SELECT * FROM entry");

    generate_response(query, &db_pool).await
}

#[get("/by-entry-uuid/{entry_uuid}/")]
async fn get_entries_from_entry(
    Path(entry_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let query = sqlx::query_as("SELECT * FROM entry WHERE uuid LIKE ?").bind(entry_uuid);

    generate_response(query, &db_pool).await
}

#[get("/by-med-uuid/{medication_uuid}/")]
async fn get_entries_from_medication(
    Path(medication_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let query =
        sqlx::query_as("SELECT * FROM entry WHERE medication_uuid LIKE ?").bind(medication_uuid);

    generate_response(query, &db_pool).await
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
