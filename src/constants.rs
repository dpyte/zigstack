pub static CONNECTION: &str = "/dev/ttyUSB0";
pub const BAUDRATE: u32 = 115200;
pub const RTSCTS: bool = false;

/// `SOF`: Start of frame indicator.
/// Start of frame indicator. This is always set to 0xFE.
pub const SOF: u8 = 0xFE;
/// `FMT_DATA_SIZE`: The length of the data field of the frame
/// The length can range from 0-250
pub const FMT_DATA_SIZE: i32 = 250;
