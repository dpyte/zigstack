use crate::commands::Command;
use crate::constants::SOF;

///
/// +-----+-----------------------------------------------+-----+
/// | SOF |           Monitor/Test Frame Format           | FCS |
/// +-----+-----------------------------------------------+-----+
///  1Byte                    3-253 Bytes                  1Byte
///
/// StartOfFrame (SOF): 0xFE
/// Monitor/Test Frame Format: Refer to GeneralFrame
/// FCS: Frame Check Sequence (computed as XOR)
pub fn calc_fcs(data: &[u8]) -> u8 {
	const INITIAL: u8 = 0;
	let res = data.iter().fold(INITIAL, |acc, &x| acc ^ x);
	res
}


///
/// GeneralFrame
/// |------Header-------|
/// +-----+------+------+-----------------------------------------+
/// | Len | Cmd0 | Cmd1 |                Data                     |
/// +-----+------+------+-----------------------------------------+
///  1Byte 1Byte  1Byte                0-250 Byte
///
/// The standard MT frame format consists of the 3-byte MT header and an optional data field of up to 250
/// bytes. Note that the upper bit (bit 7) of CMD0 is set to zero in this format. The Len element of the MT
/// header indicates the number of bytes in the DATA part of the frame.
#[derive(Debug, Clone)]
pub struct Header {
	len: u8,
	command: Command,
}


/// Extended Header
///     7 6 5 4 3 2 1 0
/// ... +---------+--+-----------------+
///     | Version |  |  Header Bytes   |
/// ... +---------+--+-----------------+
///     |-  1 Byte  -| Optional Bytes
#[derive(Debug, Clone)]
pub struct ExtendedHeader {
	// only care about the upper 5-bits
	version: u8,
	// only care about the lower 3-bits
	id: u8,
}

///
/// StandardFrame
/// The standard frame structure consists of a 3-byte header followed by a data field that can hold up to
/// 250 bytes. The header is represented by the `Header` struct and contains information about the frame,
/// such as the length and command. The data field is an array of 250 bytes, represented by `[u8; 250]`.
///
/// # Example
///
/// ```rust
/// use crate::StandardFrame;
///
/// let frame = StandardFrame {
///     header: Header {
///         len: 10,
///         command: Command::SomeCommand,
///     },
///     data: [0; 250],
/// };
/// ```
///
/// In the example above, we create a `StandardFrame` instance with a header length of 10 bytes and a command
/// of type `Command::SomeCommand`. The data field is initialized with all zeros using the `[0; 250]` syntax.
///
#[derive(Debug, Clone)]
pub struct StandardFrame {
	header: Header,
	data: Vec<u8>,
}

///
/// ExtendedFrame
///
/// The ExtendedFrame struct represents an extended frame in a communication protocol. An extended frame is an
/// extension of the standard MT frame format, consisting of the 3-byte MT header and an optional data field of
/// up to 250 bytes. The ExtendedFrame struct inherits all the fields and functionalities of the Header struct,
/// adding the capability to include additional information in the extended data field.
///
/// The standard MT frame format is as follows:
///
/// |------Header-------|
/// +-----+------+------+-----------------------------------------+
/// | Len | Cmd0 | Cmd1 |                Data                     |
/// +-----+------+------+-----------------------------------------+
///  1Byte 1Byte  1Byte               0-250 Bytes
///
/// The Len field of the MT header indicates the number of bytes in the Data part of the frame.
///
/// The ExtendedFrame struct does not introduce any new fields or methods beyond those already defined in the
/// Header struct, but it provides a more specialized representation of the extended frame.
///
#[derive(Debug, Clone)]
pub struct ExtendedFrame {

}

/// Represents a frame type of monitor.
#[derive(Debug)]
pub enum MonitorFrame {
	StandardFrame(StandardFrame),
	ExtendedFrame(ExtendedFrame),
}

/// Trait for defining the structure of a frame.
///
/// This trait provides a contract for implementing the `serialize` method, which returns a vector of bytes representing the serialized frame. It is intended
pub trait FrameStructure {
	fn serialize(&self) -> Vec<u8>;
}

impl Header {
	pub fn from_bytes(data: &[u8; 3]) -> Option<Self> {
		if data.len() > 3 || data[0] > 255 {
			return None;
		}

		let len = data[0];
		let command = Command::from_bytes(&[data[1], data[2]]);
		Some(Self{ len, command })
	}

	pub fn serialize(&self) -> Vec<u8> {
		let mut res = Vec::new();
		res.push(self.len);
		res.extend_from_slice(self.command.serialize().as_ref());
		assert_eq!(res.len(), 3);
		res
	}
}

impl FrameStructure for StandardFrame {
	fn serialize(&self) -> Vec<u8> {
		let mut res = Vec::new();
		res.extend_from_slice(self.header.serialize().as_slice());
		res.extend_from_slice(self.data.as_slice());
		res
	}
}

impl StandardFrame {
	pub fn from_bytes(data: &[u8]) -> Self {
		let header = Header::from_bytes(&[data[0], data[1], data[2]])
			.expect("invalid header frame");
		let mut data = Vec::new();

		if data.len() > 3 {
			data = Vec::from(&data.clone()[3..]);
		}
		Self { header, data }
	}

	pub fn deserialize(&self, data: &[u8]) -> Self {
		// just deserialize to Standard frame for now...
		Self::from_bytes(data)
	}
}

impl FrameStructure for MonitorFrame {
	fn serialize(&self) -> Vec<u8> {
		match self {
			MonitorFrame::StandardFrame(std_frame) => {
				let serialized_std_frame = std_frame.serialize();
				let fcs = calc_fcs(&serialized_std_frame);

				let mut res = Vec::with_capacity(1 + serialized_std_frame.len() + 1);

				res.push(SOF);
				res.extend(serialized_std_frame);
				res.push(fcs);

				res
			}
			MonitorFrame::ExtendedFrame(_) => Vec::new(),
		}
	}
}

impl MonitorFrame  {
	pub fn from_bytes(data: &[u8]) -> MonitorFrame {
		let data_len = *&data[1];

		let mut boi = &data[1..4 + data_len as usize];
		if data.len() > 3 && data.len() - 4 == data_len as usize {
			boi = &data[1..data.len() - 1]
		}
		Self::StandardFrame(StandardFrame::from_bytes(boi))
	}

	/// @params data: a complete 255 bytes data frame consisting of sof, monitor frame, and the fcs.
	/// @returns MonitorFrame
	pub fn deserialize(&self, data: &[u8]) -> MonitorFrame {
		Self::from_bytes(data)
	}

	pub fn command(&self) -> Command {
		let mut std_frame = Command::from_bytes(&[0x00, 0x00]);
		if let MonitorFrame::StandardFrame(frame) = self {
			std_frame = frame.header.command.clone();
		}
		std_frame
	}
}

