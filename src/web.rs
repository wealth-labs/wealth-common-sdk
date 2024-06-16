use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub listen: String,
	pub show_log: bool,
}

pub async fn init() -> Result<()> {
	Ok(())
}
