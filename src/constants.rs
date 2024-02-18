pub static CONNECTION: &str = "/dev/ttyUSB0";
pub const BAUDRATE: u32 = 115200;
pub const RTSCTS: bool = false;

/// `SOF`: Start of frame indicator.
/// Start of frame indicator. This is always set to 0xFE.
pub const SOF: i32 = 0xFE;
pub const DATA_START: i32 = 4;
pub const POSITION_DATA_LENGTH: i32 = 1;
pub const POSITION_CMD0: i32 = 2;
pub const POSITION_CMD1: i32 = 3;

/// `FMT_DATA_SIZE`: The length of the data field of the frame
/// The length can range from 0-250
pub const FMT_DATA_SIZE: i32 = 250;
