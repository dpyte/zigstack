use std::fmt::Debug;
use std::io::Write;

use anyhow::Result;
use crc16::*;
use tokio_serial::{SerialPortBuilderExt, SerialStream};

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

	pub async fn send_ack(&mut self, ack_num: u8) -> Result<()> {
		let ack_frame = Self::make_frame(0b10000000 | ack_num, None);
		self.port.write(&*ack_frame)?;
		Ok(())
	}

	fn make_frame(control: u8, data: Option<Vec<u8>>) -> Vec<u8> {
		// Construct a frame
		let mut frame = vec![control];

		if let Some(mut d) = data {
			frame.append(&mut d);
		}

		// Calculate crc
		let crc = State::<XMODEM>::calculate(&frame);
		let crc_arr = vec![(crc >> 8) as u8, (crc % 256) as u8];

		frame.extend_from_slice(&crc_arr);
		frame.push(0x7E);
		frame
	}
}