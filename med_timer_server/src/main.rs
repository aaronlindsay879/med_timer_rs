mod entries;
mod meds;

use actix_web::{middleware::Logger, App, HttpServer};
use log::LevelFilter;
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
            .data(pool.clone())
            .configure(meds::config)
            .configure(entries::config)
            .wrap(Logger::default())
    });

    // finally bind the server to an address and run
    server.bind("127.0.0.1:8080")?.run().await
}
