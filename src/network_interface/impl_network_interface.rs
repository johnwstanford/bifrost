use crate::network_interface::{IFF_NO_PI, IFF_TAP, NetworkInterface};

impl NetworkInterface {

    pub fn new(name: &str, mac_addr: [u8; 6], ip_addr: [u8; 4]) -> Result<Self, &'static str> {
        Ok(Self {
            name: name.to_string(),
            fd: None, mac_addr, ip_addr
        })
    }

    pub fn start(&mut self) {
        if self.fd.is_none() {
            match self.tun_tap_alloc(IFF_TAP + IFF_NO_PI, self.mac_addr, self.ip_addr) {
                Ok(fd) => self.fd = Some(fd),
                Err(e) => {
                    println!("Failed to allocate tun-tap interface due to {:?}", e);
                    println!("To make tun-tap allocation work, make sure you run as root");
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.fd.take();
    }

}