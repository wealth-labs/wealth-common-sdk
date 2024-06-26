use anyhow::Result;
use axum::{
	response::{IntoResponse, Response},
	Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Display;
use tokio::sync::mpsc::{channel, Receiver};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub listen: String,
}

pub async fn init(
	config: &Config,
	register_router: fn(app: axum::Router) -> axum::Router,
) -> Result<()> {
	let app_stop_notice = crate::app::ins().create_app_stop_notice().await;
	let (app_waiting_send, app_waiting_recv) = channel::<Result<()>>(1);
	crate::app::ins().add_app_waiting(&config.name, app_waiting_recv).await;
	let listen = config.listen.to_owned();
	tokio::spawn(async move {
		let result = run(listen, register_router, app_stop_notice).await;
		app_waiting_send.send(result).await.ok();
	});
	Ok(())
}

async fn run(
	listen: String,
	register_router: fn(app: axum::Router) -> axum::Router,
	mut notice: Receiver<()>,
) -> Result<()> {
	let app = axum::Router::new().fallback(handler_404);

	let app = register_router(app);
	let listener = tokio::net::TcpListener::bind(listen).await?;
	axum::serve(listener, app)
		.with_graceful_shutdown(async move {
			notice.recv().await;
		})
		.await?;
	Ok(())
}

async fn handler_404() -> Response {
	WebJsonResult::error(404, "resource not found").into_response()
}

#[derive(Debug)]
pub struct WebJsonResult {
	pub code: u64,
	pub msg: String,
	pub data: Value,
}

impl WebJsonResult {
	pub fn new(code: u64, msg: &str, data: Value) -> Self {
		Self { code, msg: msg.to_owned(), data }
	}

	pub fn ok(data: Value) -> Self {
		Self::new(0, "", data)
	}
	pub fn error(code: u64, msg: &str) -> Self {
		Self::new(code, msg, Value::Null)
	}
}

impl Display for WebJsonResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Code({}), Msg({:?}), data({:?})", self.code, self.msg, self.data)
	}
}

impl IntoResponse for WebJsonResult {
	fn into_response(self) -> Response {
		(
			StatusCode::OK,
			Json(json!({
				"code":self.code,
				"msg":self.msg,
				"data":self.data,
			})),
		)
			.into_response()
	}
}

impl<E> From<E> for WebJsonResult
where
	E: Into<anyhow::Error>,
{
	fn from(value: E) -> Self {
		let err: anyhow::Error = value.into();
		let err_msg = err.to_string();
		if let Ok(result) = err.downcast::<WebJsonResult>() {
			result
		} else {
			tracing::error!("{}", err_msg);
			Self::new(500, "server error", Value::Null)
		}
	}
}

pub type WebResponse = Result<WebJsonResult, WebJsonResult>;
