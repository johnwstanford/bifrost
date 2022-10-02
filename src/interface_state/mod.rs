use crate::network_interface::NetworkInterface;

mod impl_ethernet_packet;
mod impl_arp_payload;

mod impl_interface_state;

#[derive(Debug, Clone)]
pub enum EthernetPacket {
    ARP{ dst_mac: [u8; 6], src_mac: [u8; 6], payload: Vec<u8> },
    IPv4 { dst_mac: [u8; 6], src_mac: [u8; 6], payload: Vec<u8> },
    IPv6 { dst_mac: [u8; 6], src_mac: [u8; 6], payload: Vec<u8> },
    Generic{ dst_mac: [u8; 6], src_mac: [u8; 6], ether_type: [u8; 2], payload: Vec<u8> },
    Invalid(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct AddressResolutionProtocolPayload {
    pub is_request: bool,
    pub sender_hardware_address: [u8; 6],
    pub sender_protocol_address: [u8; 4],
    pub target_hardware_address: [u8; 6],
    pub target_protocol_address: [u8; 4],
}

pub struct InterfaceState {
    pub iface: NetworkInterface,
}