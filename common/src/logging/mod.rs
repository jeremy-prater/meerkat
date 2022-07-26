use chrono;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use log::error;
use std::sync::RwLock;

lazy_static! {
    static ref LOG_LEVEL: RwLock<log::LevelFilter> = RwLock::new(log::LevelFilter::Info);
}

pub fn set_log_level(level: log::LevelFilter) {
    *(LOG_LEVEL.write().unwrap()) = level;
    error!("Setting log level : {}", level);
}

pub fn init_logging() -> Result<(), fern::InitError> {
    let mut colors = ColoredLevelConfig::new();

    colors.error = Color::Red;
    colors.warn = Color::Yellow;
    colors.info = Color::Green;
    colors.debug = Color::Blue;
    colors.trace = Color::Magenta;

    let start = chrono::Local::now();

    let logger = fern::Dispatch::new()
        .filter(|metadata| metadata.level() <= *LOG_LEVEL.read().unwrap())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{timestamp} ::: {module} [{level}] > {message}",
                timestamp = {
                    let duration = chrono::Local::now() - start;

                    let hours = duration.num_hours();
                    let duration = duration
                        .checked_sub(&chrono::Duration::hours(hours))
                        .unwrap();

                    let minutes = duration.num_minutes();
                    let duration = duration
                        .checked_sub(&chrono::Duration::minutes(minutes))
                        .unwrap();

                    let seconds = duration.num_seconds();
                    let duration = duration
                        .checked_sub(&chrono::Duration::seconds(seconds))
                        .unwrap();

                    let micros = duration.num_nanoseconds().unwrap();
                    format!("{}:{:02}:{:02}.{:09}", hours, minutes, seconds, micros)
                },
                module = record.target(),
                level = colors.color(record.level()),
                message = message
            ))
        });

    logger.chain(std::io::stdout()).apply()?;

    Ok(())
}
