use std::io::Read;
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::AsRawFd;
use crate::network_interface::{NetworkInterface, pollfd, POLLIN};

#[link(name = "c")]
extern {
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
}

impl NetworkInterface {

    pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_ready_to_read()? {
            return Err("Not ready to read");
        }

        self.fd.as_mut().unwrap().read(buf)
            .map_err(|_| "Poll indicated that data was ready to read, but the read itself failed")
    }

    pub fn is_ready_to_read(&self) -> Result<bool, &'static str> {

        let mut fds: [pollfd; 1] = [pollfd {
            fd: self.fd.as_ref().ok_or("Expected self.fd to be Some")?.as_raw_fd(),
            events: POLLIN,
            revents: 0
        }];

        unsafe {
            if poll(fds.as_mut_ptr(), fds.len() as c_ulong, 0) < 0 {
                // A negative value indicates failure
                return Err("Negative return value from poll");
            }
        }

        // A positive value indicates the number of file descriptors selected

        Ok(fds[0].revents & POLLIN != 0)
    }

}