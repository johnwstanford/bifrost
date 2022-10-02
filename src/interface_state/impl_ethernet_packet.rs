use crate::interface_state::EthernetPacket;
use crate::utils::{get_2_starting_at, get_6_starting_at};

impl EthernetPacket {

    pub fn from_slice(data: &[u8]) -> Self {
        let dst_mac = match get_6_starting_at(data, 0) {
            Some(mac) => mac,
            None => {
                return Self::Invalid(data.to_vec())
            }
        };

        let src_mac = match get_6_starting_at(data, 6) {
            Some(mac) => mac,
            None => {
                return Self::Invalid(data.to_vec())
            }
        };

        let ether_type = get_2_starting_at(data, 12);

        match ether_type {
            Some([0x08, 0x00]) => Self::IPv4 { dst_mac, src_mac, payload: data[14..].to_vec() },
            Some([0x08, 0x06]) => Self::ARP { dst_mac, src_mac, payload: data[14..].to_vec() },
            Some([0x86, 0xDD]) => Self::IPv6 { dst_mac, src_mac, payload: data[14..].to_vec() },
            Some(ether_type) => Self::Generic { dst_mac, src_mac, ether_type, payload: data[10..].to_vec() },
            None => Self::Invalid(data.to_vec()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            EthernetPacket::IPv4 { dst_mac, src_mac, payload } => dst_mac.iter().copied()
                .chain(src_mac.iter().copied())
                .chain(vec![0x08, 0x00].into_iter())
                .chain(payload.iter().copied())
                .collect(),
            EthernetPacket::ARP { dst_mac, src_mac, payload } => dst_mac.iter().copied()
                .chain(src_mac.iter().copied())
                .chain(vec![0x08, 0x06].into_iter())
                .chain(payload.iter().copied())
                .collect(),
            EthernetPacket::IPv6 { dst_mac, src_mac, payload } => dst_mac.iter().copied()
                .chain(src_mac.iter().copied())
                .chain(vec![0x86, 0xDD].into_iter())
                .chain(payload.iter().copied())
                .collect(),
            EthernetPacket::Generic { dst_mac, src_mac, ether_type, payload } => dst_mac.iter().copied()
                .chain(src_mac.iter().copied())
                .chain(ether_type.iter().copied())
                .chain(payload.iter().copied())
                .collect(),
            EthernetPacket::Invalid(v) => v.clone(),
        }
    }

}

