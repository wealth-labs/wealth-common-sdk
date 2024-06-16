use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::channel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub listen: String,
	pub show_log: bool,
}

pub async fn init(
	config: &Config,
	register_router: fn(app: axum::Router) -> axum::Router,
) -> Result<()> {
	let app_stop_notice = crate::app::ins().create_app_stop_notice().await;
	let (app_waiting_send, app_waiting_recv) = channel::<Result<()>>();
	crate::app::ins().add_app_waiting(&config.name, app_waiting_recv).await;
	let listen = config.listen.to_owned();
	tokio::spawn(async move {
		let result = run(listen, register_router, app_stop_notice).await;
		app_waiting_send.send(result).ok();
	});
	Ok(())
}

async fn run(
	listen: String,
	register_router: fn(app: axum::Router) -> axum::Router,
	app_stop_notice: tokio::sync::oneshot::Receiver<()>,
) -> Result<()> {
	let app = axum::Router::new();
	let app = register_router(app);
	let listener = tokio::net::TcpListener::bind(listen).await?;
	axum::serve(listener, app)
		.with_graceful_shutdown(async move {
			app_stop_notice.await.ok();
		})
		.await?;
	Ok(())
}
