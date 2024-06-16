use anyhow::Result;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::{
	mpsc::{channel, Receiver, Sender},
	Mutex,
};

static INS: OnceCell<App> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct App {
	app_waiting: Arc<Mutex<Vec<(String, Receiver<Result<()>>)>>>,
	app_stop_notice: Arc<Mutex<Vec<Sender<()>>>>,
	stop_send: Arc<Sender<()>>,
	stop_recv: Arc<Mutex<Receiver<()>>>,
}

pub fn init() {
	INS.set(App::new()).unwrap();
}

pub fn ins<'a>() -> &'a App {
	INS.get().unwrap()
}

impl App {
	fn new() -> Self {
		let (stop_send, stop_recv) = tokio::sync::mpsc::channel::<()>(1);
		let stop_send = Arc::new(stop_send);

		{
			let stop_send = stop_send.clone();
			tokio::spawn(async move {
				tokio::signal::ctrl_c().await.ok();
				stop_send.send(()).await.ok();
			});
		}

		let app = Self {
			app_waiting: Arc::new(Mutex::new(vec![])),
			app_stop_notice: Arc::new(Mutex::new(vec![])),
			stop_send,
			stop_recv: Arc::new(Mutex::new(stop_recv)),
		};
		app
	}

	pub async fn create_app_stop_notice(&self) -> Receiver<()> {
		let (tx, rx) = channel::<()>(1);
		self.app_stop_notice.lock().await.push(tx);
		rx
	}

	pub async fn add_app_waiting(&self, name: &str, signal: Receiver<Result<()>>) {
		let mut app_waiting = self.app_waiting.lock().await;
		app_waiting.push((name.to_owned(), signal));
	}

	pub async fn stop(&self) {
		self.stop_send.send(()).await.ok();
	}

	pub async fn waiting(&self) -> Result<()> {
		tracing::info!("application running .....");
		self.stop_recv.lock().await.recv().await;
		tracing::info!("application stopping .....");
		let mut app_stop_notice = self.app_stop_notice.lock().await;
		for notice in app_stop_notice.iter_mut() {
			notice.send(()).await.ok();
		}
		let mut app_waiting = self.app_waiting.lock().await;
		for (name, signal) in app_waiting.iter_mut() {
			tracing::info!("application stopping({}) .....", name);
			let msg = match signal.recv().await {
				Some(msg) => match msg {
					Ok(_) => "OK".to_owned(),
					Err(err) => err.to_string(),
				},
				None => "OK".to_owned(),
			};
			tracing::info!("application stopped({}) : {}", name, msg);
		}
		tracing::info!("application stopped");
		Ok(())
	}
}
