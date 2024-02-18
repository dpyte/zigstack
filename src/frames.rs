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
	res as u8
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
pub struct ExtendedFrame {

}

/// Represents a Standard Frame.
pub enum MonitorFrame {
	StandardFrame(StandardFrame),
	ExtendedFrame(ExtendedFrame),
}

trait FrameStructure {
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
		let data = Vec::from(&data.clone()[3..]);
		Self { header, data }
	}

	pub fn deserialize(&self, data: &[u8]) -> Self {
		// just deserialize to Standard frame for now...
		let bytes_of_int = &data[0..3];
		let header = Header::from_bytes(&[bytes_of_int[0],
			bytes_of_int[1], bytes_of_int[2]]).expect("invalid header frame");
		let data = Vec::from(&data[3..]);
		StandardFrame { header, data }
	}
}

impl FrameStructure for MonitorFrame {
	fn serialize(&self) -> Vec<u8> {
		match self {
			MonitorFrame::StandardFrame(std_frame) => {
				let serialized_std_frame = std_frame.serialize();
				let fcs = calc_fcs(&serialized_std_frame);

				let mut res = Vec::with_capacity(1 + serialized_std_frame.len() + 1);

				res.push(SOF as u8);
				res.extend(serialized_std_frame);
				res.push(fcs);

				res
			}
			MonitorFrame::ExtendedFrame(_) => Vec::new(),
		}
	}
}

impl MonitorFrame  {
	pub fn from_bytes(data: &[u8]) {
		let boi = &data[3..253];
		let header = Header::from_bytes(&[boi[0], boi[1], boi[2]]);
	}
}

