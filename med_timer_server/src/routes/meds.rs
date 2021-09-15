use actix_web::{
    get,
    web::{self, Path},
    HttpResponse, Responder,
};
use med_timer_shared::med::Med;
use sqlx::SqlitePool;

use super::Query;

/// Generates a json response from the given query and database pool.
/// If results are found, simply return those results.
/// Otherwise serve empty json.
async fn generate_response(query: Query<'_, Med>, db_pool: &SqlitePool) -> HttpResponse {
    match query.fetch_all(db_pool).await {
        Ok(meds) => HttpResponse::Ok().json(meds),
        Err(_) => HttpResponse::Ok().json([0; 0]),
    }
}

#[get("/")]
async fn get_all_meds(db_pool: web::Data<SqlitePool>) -> impl Responder {
    log::trace!("searching database for all medications");
    let query = sqlx::query_as("SELECT * FROM medication");

    generate_response(query, &db_pool).await
}

#[get("/by-uuid/{medication_uuid}/")]
async fn get_med_by_uuid(
    Path(medication_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    log::trace!(
        "searching database for medication with uuid: `{}`",
        medication_uuid
    );
    let query = sqlx::query_as("SELECT * FROM medication WHERE uuid LIKE ?").bind(medication_uuid);

    generate_response(query, &db_pool).await
}

#[get("/by-name/{name}/")]
async fn get_med_by_name(
    Path(name): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    log::trace!("searching database for medication with name: `{}`", name);
    let query = sqlx::query_as("SELECT * FROM medication WHERE name LIKE ?").bind(name);

    generate_response(query, &db_pool).await
}

/// Adds all med services to config
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("med")
            .service(get_all_meds)
            .service(get_med_by_uuid)
            .service(get_med_by_name),
    );
}
