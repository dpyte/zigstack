
#[cfg(test)]
mod tests {
	use anyhow::Result;
	use zigstack::commands::{Command, CommandType, Subsystem};
	use zigstack::frames::{Header, MonitorFrame, StandardFrame};

	#[test]
	fn test_command_from_bytes() -> Result<()> {
		// Areq + SYS: 0b01000010
		let data = [66, 2];
		let command = Command::from_bytes(&data);

		let subsystem = command.subsystem();
		let cmd_type = command.cmd_type();

		assert_eq!(command.cmd0(), 66);
		assert_ne!(command.cmd0(), 2);
		assert_eq!(command.subsystem(), Subsystem::MacIface);
		assert_eq!(command.cmd_type(), CommandType::Areq);
		Ok(())
	}

	#[test]
	fn test_header_frame() -> Result<()> {
		// Areq + SYS: 66
		let data = [1u8, 66u8, 249u8];
		let header = Header::from_bytes(&data);
		let serialized = match header {
			None => panic!("header is None"),
			Some(v) => v
		};

		let serialized = serialized.serialize().as_slice();

		Ok(())
	}

	#[test]
	fn test_frame_serialize() {
		// Your setup code goes here. You should create a MonitorFrame instance.
	}
}
