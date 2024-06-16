use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::OnceCell;
use tokio::sync::{oneshot::Receiver, Mutex};

static INS: OnceCell<App> = OnceCell::new();

#[derive(Debug, Clone)]
struct App {
	signals: Arc<Mutex<Vec<(String, Receiver<String>)>>>,
}

pub fn init() -> Result<()> {
	INS.set(App::new()).unwrap();
	Ok(())
}

pub fn ins<'a>() -> &'a App {
	INS.get().unwrap()
}

impl App {
	fn new() -> Self {
		Self { signals: Arc::new(Mutex::new(vec![])) }
	}
	pub async fn add(&self, name: &str, signal: Receiver<String>) {
		let mut signals = self.signals.lock().await;
		signals.push((name.to_owned(), signal));
	}

	pub async fn waiting(&self) -> Result<()> {
		tokio::signal::ctrl_c().await?;
		let mut signals = self.signals.lock().await;
		for (name, signal) in signals.iter_mut() {
			println!("application stopping({}) .....", name);
			let msg = match signal.await {
				Ok(msg) => msg,
				Err(err) => err.to_string(),
			};
			println!("application stopped({}) : {}", name, msg);
		}
		println!("application stopped");
		Ok(())
	}
}
