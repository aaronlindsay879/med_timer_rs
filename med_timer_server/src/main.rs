use log::LevelFilter;
use med_timer_shared::med::Med;
use std::{error::Error, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
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
    builder.init();

    // just to test logging
    let _med = Med::new("aa".into());
    log::error!("aa!");

    Ok(())
}
