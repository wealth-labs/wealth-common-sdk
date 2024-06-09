use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::{any::AnyPoolOptions, Any, Pool};
use std::collections::HashMap;
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
}

static INS: Lazy<RwLock<HashMap<String, Pool<Any>>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn ins(key: Option<&str>) -> Pool<Any> {
    INS.read().await.get(key.unwrap_or("")).cloned().unwrap()
}

pub async fn init(conf: &Config, key: Option<&str>) -> Result<()> {
    sqlx::any::install_default_drivers();

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

    let pool = AnyPoolOptions::new()
        .min_connections(conf.min_conn)
        .max_connections(conf.max_conn)
        .connect(&dns)
        .await?;

    INS.write()
        .await
        .insert(key.unwrap_or("").to_string(), pool);

    Ok(())
}
