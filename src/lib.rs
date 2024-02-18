extern crate core;

use std::time::SystemTime;

use anyhow::Result;
use chrono::prelude::*;

pub mod uart;
pub mod constants;
pub mod types;
pub mod znp;
pub mod frames;
pub mod commands;

fn setup_logger() -> Result<(), fern::InitError> {
	let save_as = format!("zigstack_{}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"[{} {} {}] {}",
				humantime::format_rfc3339_seconds(SystemTime::now()),
				record.level(),
				record.target(),
				message
			))
		})
		.level(log::LevelFilter::Debug)
		.chain(std::io::stdout())
		.chain(fern::log_file(save_as)?)
		.apply()?;
	Ok(())
}


