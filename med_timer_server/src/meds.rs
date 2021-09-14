use actix_web::{get, web, HttpResponse, Responder};
use med_timer_shared::med::Med;
use sqlx::SqlitePool;

/// Generates a json response from the given uuid and database pool.
/// If results are found, simply return those results.
/// Ohterwise serve empty json.
async fn generate_response(uuid: Option<String>, db_pool: &SqlitePool) -> HttpResponse {
    log::trace!(
        "searching database for medication: `{}`",
        uuid.clone().unwrap_or_else(|| "*".to_string())
    );

    let db_response: Result<Vec<Med>, sqlx::Error> =
        sqlx::query_as("SELECT * FROM medication WHERE uuid LIKE ?")
            .bind(uuid.unwrap_or_else(|| "%".to_string()))
            .fetch_all(db_pool)
            .await;

    match db_response {
        Ok(meds) => HttpResponse::Ok().json(meds),
        Err(_) => HttpResponse::Ok().json([0; 0]),
    }
}

#[get("/med")]
async fn get_all_meds(db_pool: web::Data<SqlitePool>) -> impl Responder {
    generate_response(None, db_pool.as_ref()).await
}

#[get("/med/{medication_uuid}")]
async fn get_med(
    web::Path(medication_uuid): web::Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    generate_response(Some(medication_uuid), db_pool.as_ref()).await
}

/// Adds all med services to config
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_med).service(get_all_meds);
}
