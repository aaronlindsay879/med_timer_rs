#![feature(async_closure)]

mod routes;
use actix_web::{middleware, App, HttpServer};
use anyhow::anyhow;
use log::LevelFilter;
use paperclip::actix::OpenApiExt;
use routes::{entries, meds};
use sqlx::SqlitePool;
use std::{io::Write, path::Path};

struct EnvVars {
    pub database_url: String,
    pub logging_level: LevelFilter,
}

impl EnvVars {
    /// Reads env vars from a file if it exists, and then fetch all the env vars needed.
    pub fn new() -> anyhow::Result<Self> {
        // load env vars from file, discard result
        // this is as it's totally okay for it to not load a dotenv file, as env vars can be passed in in other ways
        let _ = dotenv::dotenv();

        // then try and read all needed env vars
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(path) if Path::new(&path).exists() => path,
            Ok(_) => return Err(anyhow!("DATABASE_URL points to a non-existent file")),
            Err(_) => return Err(anyhow!("DATABASE_URL not set")),
        };

        // if LOGGING_LEVEL is not set, just default to info
        let logging_level = std::env::var("LOGGING_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .parse()
            .map_err(|_| anyhow!(r#"LOGGING_LEVEL could not be parsed: try setting to one of ["off", "error", "warn", "info", "debug", "trace"]"#))?;

        Ok(Self {
            database_url,
            logging_level,
        })
    }

    /// Returns the correct logging level for the `sqlx::query` module.
    pub fn sqlx_query_logging_level(&self) -> LevelFilter {
        match self.logging_level {
            LevelFilter::Info => LevelFilter::Warn,
            _ => self.logging_level,
        }
    }
}

fn initialise_logger(env: &EnvVars) {
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

    // set filter levels - sqlx::query logs at a weird level so needs a special rule to make logs tidier
    builder.filter_level(env.logging_level);
    builder.filter_module("sqlx::query", env.sqlx_query_logging_level());
    builder.init();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let env = EnvVars::new()?;
    initialise_logger(&env);

    let pool = SqlitePool::connect(&env.database_url).await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .data(pool.clone())
            .wrap_api()
            .with_json_spec_at("/spec/")
            .configure(meds::config)
            .configure(entries::config)
            .build()
    });

    // finally bind the server to an address and run
    server.bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}
