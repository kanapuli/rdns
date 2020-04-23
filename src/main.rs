use std::io::Result;
use std::io::{Error, ErrorKind};

fn main() {
    println!("Hello DNS");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResponseCode {
    pub fn from_number(code: u8) -> ResponseCode {
        match code {
            1 => ResponseCode::FORMERR,
            2 => ResponseCode::SERVFAIL,
            3 => ResponseCode::NXDOMAIN,
            4 => ResponseCode::NOTIMP,
            5 => ResponseCode::REFUSED,
            0 | _ => ResponseCode::NOERROR,
        }
    }
}
//DNSPacketBuffer is the representation of a DNS Packet
pub struct DNSPacketBuffer {
    buffer: [u8; 512],
    position: usize,
}

impl DNSPacketBuffer {
    //new() gives a fresh buffer to hold the DNS packet contents and relative position
    fn new() -> DNSPacketBuffer {
        DNSPacketBuffer {
            buffer: [0; 512],
            position: 0,
        }
    }

    //position gives the current postion in the buffer
    fn position(&self) -> usize {
        self.position
    }

    fn step(&mut self, step: usize) -> Result<()> {
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

    fn get(&mut self, pos: usize) -> Result<u8> {
        if pos > 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        };
        Ok(self.buffer[pos])
    }

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len > 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        };
        Ok(&self.buffer[start..len + start as usize])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let response = (self.read()? as u16) << 8 | self.read()? as u16;
        Ok(response)
    }

    fn read_u32(&mut self) -> Result<u32> {
        let response = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | (self.read()? as u32);
        Ok(response)
    }

    //read_qname reads the domain name from the DNS packet buffer
    fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        //reading_qname includes jump. Hence keep a local copy of position as opposed
        //to using the position within the struct.
        let mut pos = self.position();
        //to track if we have jumped or not
        let mut jumped = false;

        //delimiter to be appended for each label.
        let mut delimiter = "";
        loop {
            let len = self.get(pos)?;

            //if len has the two most significant bits set,
            //it represents a jump to some other
            //offset in the packet
            //nice way to check if the first 2 significant bits are set
            if (len & 0xC0) == 0xC0 {
                //update the buffer position to a point past the current label
                if !jumped {
                    self.seek(pos + 2)?;
                }

                //read another byte,calculate offset and jump position
                //by updating the local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                //Indicate that the jump has been performed
                jumped = true;
            } else {
                //move a single byte forward
                pos += 1;
                //Domain names terminated with an empty label of length 0 so if
                //the length is zero, the task is done
                if len == 0 {
                    break;
                }
                //append the delimiter to our output buffer first
                outstr.push_str(delimiter);

                //Extract the actual ASCII bytes and append them to outstr
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delimiter = ".";
                pos += len as usize;
            }
        }
        //If a jump has been performed, we have already modified the buffer state
        //and should not do again
        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }
}

#[derive(Debug,Clone)]
pub struct DnsHeader {
    pub id: u16, //unique id present on dns request/ response
    pub recursion_desired: bool,// 1 bit
    pub truncated_message: bool,// 1 bit
    pub authoritative_answer: bool,// 1 bit
    pub opcode: u8, //4 bits
    pub response: bool, // 1 bit
    pub rescode: ResponseCode,
    pub checking_disabled: bool, //1 bit
    pub authed_data: bool , //1 bit
    pub z: bool, // 1 bit
    pub recursion_available: bool, // 1 bit

    pub questions: u16, //16 bit
    pub answers: u16, //16 bit
    pub authoritative_entries: u16, //16 bit
    pub resource_entries: u16 //16 bits


}