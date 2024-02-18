
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CommandType {
	Poll = 0x0,
	Sreq = 0x1,
	Areq = 0x2,
	Srsp = 0x3,
	Res4 = 4,
	Res5 = 5,
	Res6 = 6,
	Res7 = 7,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Subsystem {
	RpcErrIface = 0x00,
	SysIface = 0x01,
	MacIface = 0x02,
	NwkIface = 0x03,
	AfIface = 0x04,
	ZdoIface = 0x05,
	SimpleIface = 0x06,
	UtilIface = 0x07,
	DebugIface = 0x08,
	AppIface = 0x09,
	AppConfig = 0x0F,
	GreenPwr = 0x15,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubsystemName {
	RpcErr   = 0x00,
	SysIface = 0x01,
	MacIface = 0x02,
	UtilIface = 0x07,
	Reserved = 0x31,
}

/// MT Command
///
/// Return size: u16
///
/// The command codes consist of two bytes, “Cmd0” and “Cmd1”, as illustrated in the following figure.
/// “Cmd0” encodes the command Type in bits[7:5] and the MT Subsystem in bits[4:0]. “Cmd1” provides the
/// 8-bit command ID code for the specified Subsystem. The “Cmd0” byte is transmitted first.
#[derive(Debug, Copy, Clone)]
pub struct Command {
	cmd0: u8,
	cmd1: u8,
}

impl Command {

	/// create an instance of self using a single 16-bit data
	pub fn from_1u16(data: u16) -> Command {
		// cmd0: 0b1111111100000000
		// cmd1: 0b0000000011111111
		let cmd0 = (data & 0b0000000011111111) as u8;
		let cmd1 = ((data & 0b1111111100000000) >> 8) as u8;
		Self { cmd0, cmd1 }
	}

	pub fn from_bytes(data: &[u8; 2]) -> Self {
		let cmd0 = data[0];
		let cmd1 = data[1];
		Self { cmd0, cmd1 }
	}

	pub fn subsystem(&self) -> Subsystem {
		Self::convert_to_subsystem(self.cmd0 & 0x1F)
	}

	pub fn cmd_type(&self) -> CommandType {
		Self::convert_to_type((self.cmd0 & 0b11100000) >> 5)
	}

	pub fn id(&self) -> Option<SubsystemName> {
		if self.cmd0 > 31 {  return None;  }
		Some(match self.cmd0 {
			0x00 => SubsystemName::RpcErr,
			0x01 => SubsystemName::SysIface,
			0x02 => SubsystemName::MacIface,
			0x07 => SubsystemName::UtilIface,
			_   => SubsystemName::Reserved,
		})
	}

	pub fn cmd0(&self) -> u8 {
		self.cmd0
	}

	pub fn cmd1(&self) -> u8 {
		self.cmd1
	}

	pub fn serialize(&self) -> [u8; 2] {
		[self.cmd0, self.cmd1]
	}

	pub fn convert_to_subsystem(data: u8) -> Subsystem {
		match data {
			0x00 => Subsystem::RpcErrIface,
			0x01 => Subsystem::SysIface,
			0x02 => Subsystem::MacIface,
			0x03 => Subsystem::NwkIface,
			0x04 => Subsystem::AfIface,
			0x05 => Subsystem::ZdoIface,
			0x06 => Subsystem::SimpleIface,
			0x07 => Subsystem::UtilIface,
			0x08 => Subsystem::DebugIface,
			0x09 => Subsystem::AppIface,
			0x0F => Subsystem::AppConfig,
			0x15 => Subsystem::GreenPwr,
			_ => { Subsystem::RpcErrIface }
		}
	}

	pub fn convert_to_type(data: u8) -> CommandType {
		match data {
			0x0 => CommandType::Poll,
			0x1 => CommandType::Sreq,
			0x2 => CommandType::Areq,
			0x3 => CommandType::Srsp,
			4 => CommandType::Res4,
			5 => CommandType::Res5,
			6 => CommandType::Res6,
			7 => CommandType::Res7,
			_ => { CommandType::Res7 }
		}
	}
}


