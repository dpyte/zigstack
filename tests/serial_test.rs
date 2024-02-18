use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
};
use tokio_serial::SerialPortBuilderExt;

#[cfg(test)]
mod tests {
	use std::io;
	use tokio_test;
	use tokio_util::bytes::BytesMut;
	use tokio_util::codec::{Decoder, Encoder};
	use anyhow::Result;
	use zigstack::uart::Serial;

	struct LineCodec;

	impl Decoder for LineCodec {
		type Item = String;
		type Error = io::Error;

		fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
			let newline = src.as_ref().iter().position(|b| *b == b'\n');
			if let Some(n) = newline {
				let line = src.split_to(n + 1);
				return match std::str::from_utf8(line.as_ref()) {
					Ok(s) => Ok(Some(s.to_string())),
					Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
				};
			}
			Ok(None)
		}
	}

	impl Encoder<String> for LineCodec {
		type Error = io::Error;

		fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
			Ok(())
		}
	}

	#[tokio::test]
	async fn test_serial_connection() -> Result<()> {
		let mut serial = Serial::new().await?;


		let frm_num = (2 & 0x70) >> 4;
		let retx = (2 & 0x08) >> 3;
		let recv_seq = (frm_num + 1) & 7;

		serial.send_ack(recv_seq).await?;



		Ok(())
	}
}