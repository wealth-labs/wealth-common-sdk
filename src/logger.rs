use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use time::macros::format_description;
use time::UtcOffset;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::Subscriber;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub level: String,
    pub target: bool,
    pub thread: bool,
    pub ansi: bool,
    pub line_number: bool,
}

pub fn init(conf: &Config) -> Result<()> {
    let timer = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let log_level = LevelFilter::from_str(&conf.level)?;

    let (log_writer, guard) = non_blocking(std::io::stdout());

    let log_subscriber = Subscriber::builder()
        .with_max_level(log_level)
        .with_writer(log_writer)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_level(true)
                .with_file(false)
                .with_source_location(false)
                .with_target(conf.target)
                .with_thread_ids(conf.thread)
                .with_thread_names(conf.thread)
                .with_ansi(conf.ansi)
                .with_line_number(conf.line_number)
                .with_timer(timer)
                .compact(),
        )
        .finish();

    set_global_default(log_subscriber)?;

    std::mem::forget(guard);

    Ok(())
}
