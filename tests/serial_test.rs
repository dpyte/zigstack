use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
};
use tokio_serial::SerialPortBuilderExt;

#[cfg(test)]
mod tests {
	use std::time::Duration;
	use tokio_test;
	use anyhow::Result;
	use more_asserts::assert_ge;
	use zigstack::frames::{FrameStructure, MonitorFrame};
	use zigstack::uart::Serial;

	#[tokio::test]
	async fn test_serial_connection() -> Result<()> {
		let test_buffer = [0xFEu8, 0x00, 0x21, 0x01, 0x20];

		let mut serial = Serial::new().await?;
		serial.write(&test_buffer).await?;

		tokio::time::sleep(Duration::from_millis(3)).await;

		let mut resp = serial.read().await?;
		if resp[resp.len() - 1] == 0 {
			resp = resp[0..resp.len()-1].to_owned()
		}

		let frame = MonitorFrame::from_bytes(resp.as_slice());
		let subsystem = frame.command().subsystem();
		let fr_type = frame.command().cmd_type();

		let resp = frame.serialize();
		assert_ge!(resp.len(), 4);
		Ok(())
	}
}