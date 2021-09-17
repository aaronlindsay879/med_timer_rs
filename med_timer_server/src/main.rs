#![feature(async_closure)]

mod routes;
use log::LevelFilter;
use routes::{entries, meds};

use actix_web::{middleware, App, HttpServer};
use paperclip::actix::OpenApiExt;
use sqlx::SqlitePool;
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env vars from file, discard result
    // this is as it's totally okay for it to not load a dotenv file, as env vars can be passed in in other ways
    let _ = dotenv::dotenv();

    let database_url = std::env::var("DATABASE_URL").expect("no database url");
    let logging_level = std::env::var("LOGGING_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .expect("invalid logging level");

    // simple workaround to stop sqlx::query info-level logs showing up at info level, since they should be at debug
    let sqlx_query_logging_level = match logging_level {
        LevelFilter::Info => LevelFilter::Warn,
        _ => logging_level,
    };

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
    builder.filter_level(logging_level);
    builder.filter_module("sqlx::query", sqlx_query_logging_level);
    builder.init();

    // then create database pool, mapping to io error if needed
    let pool = SqlitePool::connect(&database_url)
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
