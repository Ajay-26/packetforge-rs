use crate::packet::Packet;

pub trait PacketSource {
    fn stream() -> Packet;
}
