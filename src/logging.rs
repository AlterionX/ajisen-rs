use chrono;
use fern;
use log;
use log::warn;
use std::str::FromStr;

use crate::settings;

pub fn setup(logging: &settings::Logging) -> Result<(), fern::InitError> {
    let log_level = log::LevelFilter::from_str(&logging.level).unwrap_or_else(|_| {
        warn!("Invalid log level specified. Defaulting to `info`");
        log::LevelFilter::Info
    });

    fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = fern::colors::ColoredLevelConfig::new()
                .debug(fern::colors::Color::Magenta)
                .trace(fern::colors::Color::Blue)
                .info(fern::colors::Color::Green)
                .warn(fern::colors::Color::Yellow)
                .error(fern::colors::Color::Red);

            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log_level)
        .level_for("h2", log::LevelFilter::Info)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("rustls", log::LevelFilter::Info)
        .level_for("tokio_reactor", log::LevelFilter::Info)
        .level_for("reqwest", log::LevelFilter::Info)
        .level_for("tungstenite", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
