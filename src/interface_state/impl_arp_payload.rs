use crate::interface_state::AddressResolutionProtocolPayload;
use crate::utils::{get_2_starting_at, get_4_starting_at, get_6_starting_at};

impl AddressResolutionProtocolPayload {

    pub fn from_slice(data: &[u8]) -> Result<Self, &'static str> {
        let htype = get_2_starting_at(data, 0).ok_or("Too few bytes for ARP")?;
        let ptype = get_2_starting_at(data, 2).ok_or("Too few bytes for ARP")?;
        let sizes = get_2_starting_at(data, 4).ok_or("Too few bytes for ARP")?;

        if htype != [0x00, 0x01] || ptype != [0x08, 0x00] || sizes != [0x06, 0x04] {
            // println!("htype={:X?} ptype={:X?} sizes={:X?}", htype, ptype, sizes);
            return Err("Unexpected parameters for ARP message");
        }

        let is_request = match get_2_starting_at(data, 6).ok_or("Too few bytes for ARP")? {
            [0x00, 0x01] => true,
            [0x00, 0x02] => false,
            _ => {
                return Err("Unexpected value in the operation field for ARP packet");
            }
        };

        let sender_hardware_address = get_6_starting_at(data, 8).ok_or("Too few bytes for ARP")?;
        let sender_protocol_address = get_4_starting_at(data, 14).ok_or("Too few bytes for ARP")?;
        let target_hardware_address = get_6_starting_at(data, 18).ok_or("Too few bytes for ARP")?;
        let target_protocol_address = get_4_starting_at(data, 24).ok_or("Too few bytes for ARP")?;

        Ok(Self {
            is_request,
            sender_hardware_address,
            sender_protocol_address,
            target_hardware_address,
            target_protocol_address
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![0x00, 0x01, 0x08, 0x00, 0x06, 0x04].into_iter()
            .chain(match self.is_request { true => vec![0, 1], false => vec![0, 2] }.into_iter())
            .chain(self.sender_hardware_address.iter().copied())
            .chain(self.sender_protocol_address.iter().copied())
            .chain(self.target_hardware_address.iter().copied())
            .chain(self.target_protocol_address.iter().copied())
            .collect()
    }
}