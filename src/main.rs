use std::io::Result;
use std::io::{Error, ErrorKind};

fn main() {
    println!("Hello DNS");
}
//DNSPacketBuffer is the representation of a DNS Packet
pub struct DNSPacketBuffer {
    buffer: [u8;512],
    position: usize,
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

    fn step(&mut self , step: usize) -> Result<()> {
        //ToDo: Check if the position exceeds greater than 512 bytes
        self.position += step;
        Ok(())
    }

    fn seek(&mut self, seek: usize) -> Result<()> {
        self.position = seek;
        Ok(())
    }

    //read reads a single byte from  the PacketBuffer
    fn read(&mut self) -> Result<u8> {
        if self.position > 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        let response = self.buffer[self.position];
        self.position += 1;
        Ok(response)
    }

    fn get(&mut self, pos: usize) ->Result<u8>{
        if pos > 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        };
        Ok(self.buffer[pos])

    }

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >  512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        };
        Ok(&self.buffer[start..len + start as usize])

    }

    fn read_u16(&mut self) -> Result<u16> {
        let response = (self.read()? as u16) << 8 | self.read()? as u16; 
        Ok(response)
    }
} 
