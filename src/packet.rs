
pub enum LinkType {
    Ethernet, 
    Linux,
    RawIP,
    Loopback,
    Other(u32),
}

pub struct PacketMetadata {
    pub timestamp_s: u64
    pub timestamp_ns: u32,
    pub link_type: LinkType,
    pub captured_len: usize,
    pub original_len: usize,    
}

pub struct Packet {
    pub metadata: PacketMetadata,
    data: Vec<u8>,
}
