use crate::packet::{Packet, PacketMetadata};
use std::vec::Vec;

pub trait ProcessorBlock {
    fn process_packet(self: &mut Self, packet: &Packet) -> ();
}

pub struct Processor {
    pub processor_blocks: Vec<Box<dyn ProcessorBlock>>,
}

impl Processor {
    pub fn new(blocks: Vec<Box<dyn ProcessorBlock>>) -> Self {
        return Processor {
            processor_blocks: blocks,
        };
    }

    pub fn process_packet(self: &mut Self, packet: Packet) -> () {
        for processor in self.processor_blocks.iter_mut() {
            processor.process_packet(&packet);
        }
    }
}

struct MetadataCollector {
    pub metadata_table: Vec<PacketMetadata>,
}

impl ProcessorBlock for MetadataCollector {
    fn process_packet(self: &mut Self, packet: &Packet) {
        self.metadata_table.push(packet.get_metadata().clone());
    }
}

impl MetadataCollector {
    pub fn new() -> Self {
        return MetadataCollector {
            metadata_table: Vec::<PacketMetadata>::new(),
        };
    }

    pub fn get_table(self: &Self) -> &[PacketMetadata] {
        return &(self.metadata_table);
    }
}
