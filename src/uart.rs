use std::fmt::Debug;
use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio_serial::{Error, SerialPortBuilderExt, SerialStream};

use crate::constants::{BAUDRATE, CONNECTION};

pub struct Serial {
	port: SerialStream,
}

impl Serial {
	pub async fn new() -> Result<Self> {
		let mut port = tokio_serial::new(CONNECTION, BAUDRATE)
			.open_native_async()?;
		port.set_exclusive(false).expect("Unable to set serial port exclusive to false");
		Ok(Self { port })
	}

	pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
		self.port.write_all(data).await?;
		Ok(())
	}

	pub async fn read(&mut self) -> Result<Vec<u8>> {
		let mut buffer = [0u8; 8];
		let bytes_read = self.port.try_read(&mut buffer)?;
		if bytes_read > 0 {
			return Ok(Vec::from(&buffer));
		} else if bytes_read == 0{
			return Ok(Vec::new());
		}
		Ok(Vec::from(&[255]))
	}

	pub async fn close(&mut self) -> Result<()> {
		Ok(self.port.shutdown().await?)
	}
}