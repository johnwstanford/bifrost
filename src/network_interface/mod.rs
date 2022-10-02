use std::fs::File;
use std::os::raw::{c_int, c_short};

const IFF_UP: c_int    = 0x1;
const IFF_TAP: c_int   = 0x2;
const IFF_NO_PI: c_int = 0x1000;

const POLLIN: c_short = 0x0001;

mod impl_network_interface;
mod impl_network_interface_tun_tap;
mod impl_network_interface_read;

pub struct NetworkInterface {
    pub name: String,
    pub mac_addr: [u8; 6],
    pub ip_addr: [u8; 4],
    pub fd: Option<File>,
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

