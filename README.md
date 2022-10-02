# bifrost

This project is a demo of TUN/TAP functionality in Linux.  It sets up two interfaces: `bifrost0` and `bifrost1`.  The `bifrost0` interface is assigned 192.168.75.1 and `bifrost1` is assigned 192.168.76.1.

## Basic Build and Test Procedure

This code has been testing in Ubuntu 22.04 with Linux kernel 5.15.0-47-generic and Rust 1.63.

- Build using `cargo build --release --examples`
- Run using `sudo ./target/release/examples/00_two_way_bridge`

In a different terminal:
- Verify two new interfaces using `ifconfig`
- Ping one way using `ping 192.168.75.2` (expected to pass)
- Ping the other way using `ping 192.168.76.2` (expected to pass)
- Try to connect over SSH using `ssh 192.168.75.2` (currently fails; not sure why)

## Network Interface Startup

The network interface is set up in `NetworkInterface::tun_tap_alloc`:

- Open a file descriptor for `dev/net/tun`
- Call `ioctl` (#1) to assign a name to the new interface and read it back
- Call `ioctl` (#2) to assign a MAC address as read it back (#3)
- Open a socket and use it for another `ioctl` (#4) to bring the interface up.
- Call `ioctl` (#5) to assign an IP address to the interface
- Call `ioctl` (#6) to assign a netmask to the interface