use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use bifrost::interface_state::InterfaceState;
use bifrost::network_interface::NetworkInterface;

fn main() -> Result<(), &'static str> {

    // IP addresses are mapped and checksums are recalculated before being put in this buffer
    // The items in this vector are ready to pull right off the queue and write straight to the file descriptor
    let buffer0_to_1: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    let buffer1_to_0: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));

    let is_active: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let is_active_send = is_active.clone();
    let buffer0_to_1_send = buffer0_to_1.clone();
    let buffer1_to_0_send = buffer1_to_0.clone();
    std::thread::spawn(move || {

        let iface0 = NetworkInterface::new(
            "custom0", [0xA, 0xB, 0xC, 0xD, 0xE, 0], [192, 168, 75, 1]).unwrap();
        let mut state0 = InterfaceState::new(iface0);
        let mut buff0: Vec<u8> = vec![0; 8192];

        while is_active_send.load(Ordering::Acquire) {

            if let Ok(n) = state0.iface.try_read(&mut buff0) {
                if n > 0 {
                    for response in state0.process_packet(&buff0[..n], 1) {
                        buffer0_to_1_send.lock().unwrap().push_back(response);
                    }
                }
            }

            while let Some(response) = buffer1_to_0_send.lock().unwrap().pop_front() {
                state0.iface.fd.as_mut().unwrap().write_all(&response).unwrap();
            }
        }

    });

    let iface1 = NetworkInterface::new(
        "custom1", [0xA, 0xB, 0xC, 0xD, 0xE, 1], [192, 168, 76, 1])?;
    let mut state1 = InterfaceState::new(iface1);
    let mut buff0: Vec<u8> = vec![0; 8192];

    while is_active.load(Ordering::Acquire) {

        if let Ok(n) = state1.iface.try_read(&mut buff0) {
            if n > 0 {
                for response in state1.process_packet(&buff0[..n], 0) {
                    buffer1_to_0.lock().unwrap().push_back(response);
                }
            }
        }

        while let Some(response) = buffer0_to_1.lock().unwrap().pop_front() {
            state1.iface.fd.as_mut().unwrap().write_all(&response).unwrap();
        }
    }

    Ok(())

}
