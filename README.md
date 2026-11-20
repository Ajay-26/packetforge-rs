# Packetforge-rs

This package is a packet capture tool, that is built in rust and designed to be readable by a non-technical person. The goal is for people 
to have a clear idea of what traffic their machines are going through. 

So the interfaces are:
- Packet: Fixed unified format for all packets. As for layer this is for, it can be TCP or http. I am inclined to do TCP for now, but eventually would like to add HTTP analyses to the analysis section (comes later, as future scope)
- Capture: This could be a linux packet capture or windows or macos. It could also just be a pcap file. Basically it takes a stream as input, and streams out packets in a fixed format
- Processor: Takes a stream of packets as input, updates state
- Processing: Has a list of Processors.
- Graphics: Takes analytics, renders them

For http: we will inherit from the Processing trait. The Packet is going to be a struct that is passed around. The packet is moved from the capture into Processing, not copied. Processing owns the packet eventually, and is free to do as it likes with it. 
