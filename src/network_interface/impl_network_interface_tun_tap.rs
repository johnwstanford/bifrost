

use std::fs::{File, OpenOptions};
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::AsRawFd;
use crate::cpp_interop::{populate_cstr, collect_cstr};
use crate::network_interface::{IFF_UP, NetworkInterface};

#[link(name = "c")]
extern {
    fn ioctl(fd: c_int, request: c_ulong, args: *mut u8) -> c_int;
    fn socket(domain: c_int, sock_type: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
}

const AF_INET: c_int = 0x2;
const SOCK_DGRAM: c_int = 0x2;

const ARPHRD_ETHER: c_int = 0x1;

const TUNSETIFF: c_ulong = 0x400454ca;
const SIOCSIFFLAGS: c_ulong  = 0x8914;
const SIOCSIFADDR: c_ulong  = 0x8916;
const SIOCSIFNETMASK: c_ulong  = 0x891c;
const SIOCSIFHWADDR: c_ulong = 0x8924;
const SIOCGIFHWADDR: c_ulong = 0x8927;

const NETMASK: [u8; 4] = [255, 255, 255, 0];

impl NetworkInterface {

    pub fn tun_tap_alloc(&self, flags: i32, mac_addr: [u8; 6], ip_addr: [u8; 4]) -> Result<File, &'static str> {

        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")
            .map_err(|_| "Unable to open /dev/net/tun")?;
        let fd = f.as_raw_fd();

        println!("Creating network interface named {}", &self.name);

        // The C headers define a 40-byte type with a confusing structure made up of unions, so
        // to avoid confusion, it's easiest to just use a 40-byte array and be very explicit about
        // what we're doing at a byte level
        let mut ifr: [u8; 40] = [0; 40];

        // Assign the interface name
        unsafe {
            populate_cstr(ifr.as_mut_ptr(), 40, &self.name);
            ifr.get_mut(16..20).unwrap().copy_from_slice(&flags.to_le_bytes());
            if ioctl(fd, TUNSETIFF, ifr.as_mut_ptr()) != 0 {
                return Err("First call to ioctl failed in tun_tap_alloc");
            }
        }

        // Read back the interface name
        unsafe {
            let name_readback = collect_cstr(ifr.as_ptr());
            println!("Created network interface named {}", &name_readback);
        }

        // Assign the MAC address
        // let mac_addr: [u8; 6] = [0xA, 0xB, 0xC, 0xD, 0xE, mac_addr[5]];
        ifr.get_mut(16..18).unwrap().copy_from_slice(&(ARPHRD_ETHER as u16).to_le_bytes());
        ifr.get_mut(18..24).unwrap().copy_from_slice(&mac_addr);
        unsafe {
            if ioctl(fd, SIOCSIFHWADDR, ifr.as_mut_ptr()) != 0 {
                return Err("Second call to ioctl failed in tun_tap_alloc");
            }
        }

        println!("Successfully set TUNTAP interface MAC address to {:X?}", &mac_addr);

        // Read back MAC address
        unsafe {
            if ioctl(fd, SIOCGIFHWADDR, ifr.as_mut_ptr()) != 0 {
                return Err("Third call to ioctl failed in tun_tap_alloc");
            }
        }

        if ifr.get(18..24).unwrap() != &mac_addr {
            return Err("MAC address readback doesn't match expected value in tun_tap_alloc");
        }

        unsafe {
            let s = socket(AF_INET, SOCK_DGRAM, 0);

            // Bring up interface
            let flags = flags | IFF_UP;
            ifr.get_mut(16..20).unwrap().copy_from_slice(&flags.to_le_bytes());
            if ioctl(s, SIOCSIFFLAGS, ifr.as_mut_ptr()) != 0 {
                println!("Unable to bring up TUNTAP interface")
            }

            // Set IP address and netmask
            ifr.get_mut(16..20).unwrap().copy_from_slice(&AF_INET.to_le_bytes());
            ifr.get_mut(20..24).unwrap().copy_from_slice(&ip_addr);
            if ioctl(s, SIOCSIFADDR, ifr.as_mut_ptr()) != 0 {
                println!("Unable to assign IP address")
            }

            ifr.get_mut(20..24).unwrap().copy_from_slice(&NETMASK);
            if ioctl(s, SIOCSIFNETMASK, ifr.as_mut_ptr()) != 0 {
                println!("Unable to assign netmask")
            }

            close(s);
        }

        Ok(f)

    }

}