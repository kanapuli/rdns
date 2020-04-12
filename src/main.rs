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

    //position gives the current postion in the buffer
    fn position(&self) -> usize {
        self.position
    }

    fn step(&self , step: usize) -> Result<()> {
        //ToDo: Check if the position exceeds greater than 512 bytes
        self.position += step;
        Ok(())
    }

}
