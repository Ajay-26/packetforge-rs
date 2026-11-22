use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;

#[derive(Debug, Eq, PartialEq)]
pub enum PacketError {
    InvalidCapturedLength,
    InvalidOriginalLength,
    InvalidTimestamp,
}

impl Display for PacketError {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketError::InvalidCapturedLength => write!(f, "Error: Invalid Capture Length!\n"),
            PacketError::InvalidOriginalLength => write!(f, "Error: Invalid Original Length!\n"),
            PacketError::InvalidTimestamp => write!(f, "Error: Invalid Timestamp!\n"),
        }
    }
}

impl Error for PacketError {}

#[derive(Clone, Debug)]
pub enum LinkType {
    Ethernet,
    Linux,
    RawIP,
    Loopback,
    Other(u32),
}

#[derive(Clone, Debug)]
pub struct PacketMetadata {
    pub timestamp_s: u64,
    pub timestamp_ns: u32,
    pub link_type: LinkType,
    pub captured_len: usize,
    pub original_len: usize,
}

#[derive(Clone)]
pub struct Packet {
    pub metadata: PacketMetadata,
    data: Vec<u8>,
}

impl Packet {
    pub fn get_metadata(self: &Self) -> &PacketMetadata {
        return &self.metadata;
    }

    pub fn get_data(self: &Self) -> &[u8] {
        return &self.data;
    }

    pub fn len(self: &Self) -> usize {
        return self.data.len();
    }

    pub fn is_empty(self: &Self) -> bool {
        return self.data.is_empty();
    }

    pub fn new(data: Vec<u8>, metadata: PacketMetadata) -> Result<Self, PacketError> {
        if metadata.captured_len != data.len() {
            return Err(PacketError::InvalidCapturedLength);
        }

        if metadata.original_len < metadata.captured_len {
            return Err(PacketError::InvalidOriginalLength);
        }

        if metadata.timestamp_ns >= 1_000_000_000 {
            return Err(PacketError::InvalidTimestamp);
        }

        Ok(Self { data, metadata })
    }
}
