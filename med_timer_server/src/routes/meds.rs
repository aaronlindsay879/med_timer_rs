use futures::StreamExt;
use med_timer_shared::med::Med;
use paperclip::actix::{
    api_v2_operation, get,
    web::{self, Json, Path},
};
use sqlx::SqlitePool;

use crate::{query, routes::DefaultQuery};

/// Fetches up to 100 medications.
#[get("/")]
#[api_v2_operation]
async fn get_all_meds(
    db_pool: web::Data<SqlitePool>,
    queries: web::Query<DefaultQuery>,
) -> Json<Vec<Med>> {
    log::trace!("searching database for all medications");

    query!(
        pool: db_pool,
        queries: queries,
        out: Vec<Med>,
        "SELECT * FROM medication"
    )
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

    query!(
        pool: db_pool,
        out: Option<Med>,
        "SELECT * FROM medication WHERE uuid LIKE ?",
        medication_uuid
    )
}

/// Fetches up to 100 medications with the given name.
#[get("/by-name/{name}/")]
#[api_v2_operation]
async fn get_med_by_name(
    Path(name): Path<String>,
    db_pool: web::Data<SqlitePool>,
    queries: web::Query<DefaultQuery>,
) -> Json<Vec<Med>> {
    log::trace!("searching database for medication with name: `{}`", name);

    query!(
        pool: db_pool,
        queries: queries,
        out: Vec<Med>,
        "SELECT * FROM medication WHERE name LIKE ?",
        name
    )
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
