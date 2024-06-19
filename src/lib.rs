#![allow(unused)]

pub mod app;
pub use app::{init as app_init, ins as app_ins, App};

pub mod config;
pub use config::init as config_init;

pub mod logger;
pub use logger::{init as log_init, Config as LogConfig};

#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "database")]
pub use database::{init as database_init, ins as database_ins, Config as DatabaseConfig};
#[cfg(feature = "database")]
pub use sea_orm::{self, sea_query::OnConflict as _, EntityTrait as _};

#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "web")]
pub use axum::{self, response::IntoResponse as _};
#[cfg(feature = "web")]
pub use web::{init as web_init, Config as WebConfig};

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
pub use tokio::{
	self, spawn,
	sync::{
		mpsc::{channel as mpsc_channel, Receiver as MpscReceiver, Sender as MpscSender},
		Mutex,
	},
	time::sleep,
};
pub use tracing::{self, debug, error, info, instrument, warn};
