#![feature(async_closure)]

pub(crate) mod db;

use db::find_med;
use log::LevelFilter;
use sqlx::SqlitePool;
use std::{convert::Infallible, error::Error, io::Write};
use warp::{path, Filter};

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
    builder.filter_level(LevelFilter::Info);
    builder.filter_module("sqlx::query", LevelFilter::Warn);
    builder.init();

    // then create database pool
    let pool = SqlitePool::connect("sqlite:///home/user/prog/rust/med_timer/testing.db").await?;

    let optional_string = warp::path::param::<String>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<String>,), Infallible>((None,)) });

    let get_single_med = path!("med" / ..)
        .map(move || pool.clone())
        .and(optional_string)
        .and_then(async move |pool, uuid: Option<String>| {
            Ok::<String, Infallible>(format!("{:?}", find_med(uuid, &pool).await))
        });

    warp::serve(get_single_med)
        .run(([127, 0, 0, 1], 3030))
        .await;
    Ok(())
}
