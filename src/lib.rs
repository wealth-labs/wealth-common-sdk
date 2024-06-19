#![allow(unused)]

pub mod app;
pub mod config;
pub mod logger;

#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "database")]
pub use sea_orm::{self, sea_query::OnConflict as _, EntityTrait as _};

#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "web")]
pub use axum::{self, response::IntoResponse as _};

pub use anyhow::{self as _anyhow, anyhow, bail, ensure, Result};
pub use async_trait::async_trait;
pub use chrono::{self, DateTime, Duration};
pub use once_cell::{self, sync::OnceCell};
pub use reqwest;
pub use rust_decimal::{self, Decimal};
pub use serde::{self, Deserialize, Serialize};
pub use serde_json::{self, Map as JsonMap, Value as Json};
pub use serde_with::{self, serde_as};
pub use std::{result::Result as StdResult, str::FromStr, time::Duration as StdDuration};
pub use time;
pub use tokio::{self, spawn, sync::Mutex, time::sleep};
pub use tracing::{self, debug, error, info, instrument, warn};
