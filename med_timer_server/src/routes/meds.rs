#![allow(clippy::async_yields_async)]

use med_timer_shared::med::Med;
use paperclip::actix::{
    api_v2_operation, get,
    web::{self, Json, Path},
};
use sqlx::SqlitePool;

use crate::generate_response_functions;

generate_response_functions!(med_response<Med>);

/// Fetches up to 100 medications.
#[get("/")]
#[api_v2_operation]
async fn get_all_meds(db_pool: web::Data<SqlitePool>) -> Json<Vec<Med>> {
    log::trace!("searching database for all medications");
    let query = sqlx::query_as("SELECT * FROM medication");

    Json(med_response(query, 100, &db_pool).await)
}

/// Fetches first medications with the given UUID.
#[get("/by-uuid/{medication_uuid}/")]
#[api_v2_operation]
async fn get_med_by_uuid(
    Path(medication_uuid): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> Json<Option<Med>> {
    log::trace!(
        "searching database for medication with uuid: `{}`",
        medication_uuid
    );
    let query = sqlx::query_as("SELECT * FROM medication WHERE uuid LIKE ?").bind(medication_uuid);

    Json(med_response(query, 1, &db_pool).await.first().cloned())
}

/// Fetches up to 100 medications with the given name.
#[get("/by-name/{name}/")]
#[api_v2_operation]
async fn get_med_by_name(
    Path(name): Path<String>,
    db_pool: web::Data<SqlitePool>,
) -> Json<Vec<Med>> {
    log::trace!("searching database for medication with name: `{}`", name);
    let query = sqlx::query_as("SELECT * FROM medication WHERE name LIKE ?").bind(name);

    Json(med_response(query, 100, &db_pool).await)
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
