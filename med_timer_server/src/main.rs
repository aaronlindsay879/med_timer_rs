#![feature(async_closure)]

mod routes;
use routes::{entries, meds};

use actix_web::{middleware, App, HttpServer};
use log::LevelFilter;
use paperclip::actix::OpenApiExt;
use sqlx::SqlitePool;
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    // then create database pool, mapping to io error if needed
    let pool = SqlitePool::connect("sqlite:///home/user/prog/rust/med_timer/testing.db")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .wrap_api()
            .with_json_spec_at("/spec/")
            .configure(meds::config)
            .configure(entries::config)
            .build()
    });

    // finally bind the server to an address and run
    server.bind("127.0.0.1:8080")?.run().await
}
