#![feature(async_closure)]

pub(crate) mod db;

use db::*;
use log::LevelFilter;
use sqlx::SqlitePool;
use std::{convert::Infallible, error::Error, io::Write};
use warp::{path, reply::Json, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create logger with format `TIMESTAMP [LEVEL] LOG`
    let mut builder = env_logger::builder();
    builder.format(|buf, record| {
        writeln!(
            buf,
            "{} [{:<5}] {}",
            buf.timestamp(),
            buf.default_styled_level(record.level()),
            record.args()
        )
    });
    builder.filter_level(LevelFilter::Trace);
    builder.filter_module("sqlx::query", LevelFilter::Warn);
    builder.init();

    // then create database pool
    let pool = SqlitePool::connect("sqlite:///home/user/prog/rust/med_timer/testing.db").await?;

    // create route for optionally matching a single string
    let optional_string = warp::path::param()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<String>,), Infallible>((None,)) });

    // create route for getting medications - if a string is passed, it's used to search for medication UUID
    let get_med = path!("med" / ..)
        .and(with_db(pool.clone()))
        .and(optional_string)
        .and_then(async move |pool, uuid| -> Result<Json, Infallible> {
            match find_med(uuid, &pool).await {
                Ok(meds) => Ok(warp::reply::json(&meds)),
                Err(_) => Ok(warp::reply::json(&[0; 0])),
            }
        });

    // create route for entries - first string is entry UUID, second is medication UUID
    let get_entry = path!("entry" / ..)
        .and(with_db(pool.clone()))
        .and(optional_string)
        .and(optional_string)
        .and_then(
            async move |pool, entry_uuid, medication_uuid| -> Result<Json, Infallible> {
                match find_entries(entry_uuid, medication_uuid, &pool).await {
                    Ok(entries) => Ok(warp::reply::json(&entries)),
                    Err(_) => Ok(warp::reply::json(&[0; 0])),
                }
            },
        );

    let routes = warp::get().and(get_med.or(get_entry));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// simple filter to make it possible to share db pool
fn with_db(
    pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
