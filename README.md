# bifrost

This project is a demo of TUN/TAP functionality in Linux.  It sets up two interfaces: `bifrost0` and `bifrost1`.  The `bifrost0` interface is assigned 192.168.75.1 and `bifrost1` is assigned 192.168.76.1.

## Network Interface Startup

The network interface is set up in `NetworkInterface::tun_tap_alloc`:

- Open a file descriptor for `dev/net/tun`
- Call `ioctl` (#1) to assign a name to the new interface and read it back
- Call `ioctl` (#2) to assign a MAC address as read it back (#3)
- Open a socket and use it for another `ioctl` (#4) to bring the interface up.