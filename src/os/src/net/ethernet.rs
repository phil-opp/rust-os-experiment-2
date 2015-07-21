
#[derive(Clone, Copy)]
pub struct MacAddress([u8; 6]);

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ProtocolType {
    Ipv4 = 0x0800,
    Ipv6 = 0x86DD,
    ARP = 0x0806,
}

impl ProtocolType {
    fn network_bytes(self) -> [u8; 2] {
        let typ = (self as u16).to_be();
        [typ as u8, (typ >> 8) as u8]
    }
}

pub trait SupportedDataPacket: AsRef<[u8]> {
    fn protocol_type(&self) -> ProtocolType;
}

#[derive(Clone, Copy)]
pub struct Header {
    destination: MacAddress,
    source: MacAddress,
}

pub struct Frame(Vec<u8>);

impl Header {
    pub fn new(source: MacAddress, destination: MacAddress) -> Header {
        Header{destination: destination, source: source}
    }

    pub fn new_frame<P>(&self, data: P) -> Option<Frame> where P: SupportedDataPacket {
        if data.as_ref().len() > 1500 {
            return None;
        }

        // construct packet
        let mut frame = Vec::with_capacity(64);
        frame.push_all(&self.destination.0);
        frame.push_all(&self.source.0);
        // TODO VLAN tag
        frame.push_all(&data.protocol_type().network_bytes());
        frame.push_all(data.as_ref());

        // an ethernet frame (without preamble and sfd) must be at least 64 large
        // so the frame vec must have at least (64 - crc) 60 entries
        while frame.len() < 60 {
            frame.push(0)
        }

        Some(Frame(frame))
    }
}
