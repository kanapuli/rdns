//DNSPacketBuffer is the representation of a DNS Packet
pub struct DNSPacketBuffer {
    buffer: [0;512],
    position: usize
}

impl DNSPacketBuffer {
    
    //new() gives a fresh buffer to hold the DNS packet contents and relative position
    fn new() -> DNSPacketBuffer {
        DNSPacketBuffer{
            buffer: [0;512],
            position: 0
        }
    }
}
