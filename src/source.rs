use crate::packet::Packet;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum SourceError {
    SourceDoesNotExist,
    SourceStreamEnd,
}

impl Display for SourceError {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceError::SourceDoesNotExist => write!(f, "Error: Source does not exist!"),
            SourceError::SourceStreamEnd => write!(f, "Source Stream Ended!\n"),
        }
    }
}

impl Error for SourceError {}

pub trait PacketSource {
    fn stream(self: &mut Self) -> Result<Packet, SourceError>;
}
