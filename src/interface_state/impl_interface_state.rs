use std::io::Write;
use crate::interface_state::{AddressResolutionProtocolPayload, EthernetPacket, InterfaceState};
use crate::{rfc_793};
use crate::network_interface::NetworkInterface;
use crate::rfc_791::ipv4_header_checksum;

impl InterfaceState {

    pub fn new(mut iface: NetworkInterface) -> Self {
        iface.start();
        Self { iface }
    }

    pub fn process_packet(&mut self, pkt: &[u8], other_mac_id: u8) -> Vec<Vec<u8>> {

        let mut ans = vec![];

        match EthernetPacket::from_slice(pkt) {
            EthernetPacket::IPv4 { dst_mac, src_mac, mut payload } => {

                for idx in vec![14, 18] {
                    if payload[idx] == 0x4b {
                        payload[idx] = 0x4c;
                    } else if payload[idx] == 0x4c {
                        payload[idx] = 0x4b;
                    }
                }

                for idx in vec![15, 19] {
                    if payload[idx] == 0x01 {
                        payload[idx] = 0x02;
                    } else if payload[idx] == 0x02 {
                        payload[idx] = 0x01;
                    }
                }

                if payload[9] == 0x06 {
                    // Update TCP checksum
                    let src_addr = [payload[12], payload[13], payload[14], payload[15]];
                    let dst_addr = [payload[16], payload[17], payload[18], payload[19]];

                    println!("Update TCP checksum: {:X?} -> {:X?}", &src_addr, &dst_addr);
                    println!("TCP payload: {:X?}", &payload[20..]);

                    rfc_793::tcp_checksum(
                        &src_addr, &dst_addr,
                        &mut payload[20..]
                    ).unwrap();
                } else {
                    println!("IPv4: {:X?} -> {:X?}; non-TCP: 0x{:X}", src_mac, dst_mac, payload[9]);
                }

                ipv4_header_checksum(&mut payload[..20]).unwrap();

                ans.push(EthernetPacket::IPv4 { dst_mac, src_mac, payload }.to_bytes());
            },
            EthernetPacket::ARP { dst_mac, src_mac, payload } => {
                let result_arp = AddressResolutionProtocolPayload::from_slice(&payload);
                match result_arp {
                    Ok(arp) => {

                        let arp_resp: Vec<u8> = EthernetPacket::ARP {
                            dst_mac: src_mac,
                            src_mac: dst_mac,
                            payload: AddressResolutionProtocolPayload {
                                is_request: false,
                                sender_hardware_address: [1, 2, 3, 4, 5, other_mac_id],
                                sender_protocol_address: arp.target_protocol_address,
                                target_hardware_address: arp.sender_hardware_address,
                                target_protocol_address: arp.sender_protocol_address,
                            }.to_bytes()
                        }.to_bytes();

                        self.iface.fd.as_mut().unwrap().write_all(&arp_resp).unwrap();

                    },
                    Err(e) => {
                        println!("Unable to parse ARP packet due to {}", e);
                    }
                }
            },
            EthernetPacket::IPv6 { dst_mac:_, src_mac:_, payload:_} => {
                // Ignore IPv6 packets
            },
            EthernetPacket::Generic { ether_type, .. } => {
                println!("Generic: ether_type={:X?}", ether_type);
            }
            EthernetPacket::Invalid(_) => {

            }
        }

        ans
    }

}