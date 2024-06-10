use anyhow::Result;
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub drive: String,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub url: String,
    pub min_conn: u32,
    pub max_conn: u32,
    pub show_log: bool,
    pub slow_query: u64,
}

static INS: Lazy<RwLock<HashMap<String, DatabaseConnection>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn ins(key: Option<&str>) -> DatabaseConnection {
    INS.read().await.get(key.unwrap_or("")).cloned().unwrap()
}

pub async fn init(conf: &Config, key: Option<&str>) -> Result<()> {
    let dns = format!(
        "{}://{}:{}@{}:{}/{}{}",
        conf.drive,
        conf.username,
        conf.password,
        conf.hostname,
        conf.port,
        conf.database,
        if conf.url.is_empty() {
            "".to_string()
        } else {
            format!("?{}", conf.url)
        },
    );

    let mut opts = ConnectOptions::new(dns);
    opts.min_connections(conf.min_conn);
    opts.max_connections(conf.max_conn);
    opts.sqlx_logging(conf.show_log);
    if conf.show_log {
        opts.sqlx_logging_level(log::LevelFilter::Info);
        opts.sqlx_slow_statements_logging_settings(
            log::LevelFilter::Warn,
            Duration::from_millis(conf.slow_query),
        );
    }

    let pool = Database::connect(opts).await?;

    INS.write()
        .await
        .insert(key.unwrap_or("").to_string(), pool);

    Ok(())
}
