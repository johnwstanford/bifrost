
pub fn ipv4_header_checksum(data: &mut [u8]) -> Result<(), &'static str> {

    if data.len() != 20 {
        return Err("Expected 20 bytes");
    }

    let data_u16: &mut [u16] = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_mut_ptr() as *mut u16,
            data.len() / 2,
        )
    };

    data_u16[5] = 0;

    let mut sum = 0u32;

    for word in data_u16.iter() {
        sum += *word as u32;
        while sum > 0xFFFF {
            let carry = (sum & 0xFFFF0000) >> 16;
            sum &= 0xFFFF;
            sum += carry;
        }
    }

    let sum = -(sum as i32 + 1) as u16;

    data_u16[5] = sum as u16;

    Ok(())
}

#[test]
fn header_checksum_test1() -> Result<(), &'static str> {

    let header_original: Vec<u8> = vec![
        0x45, 0x00, 0x00, 0x73, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, 0xb8, 0x61, 0xc0, 0xa8, 0x00, 0x01,
        0xc0, 0xa8, 0x00, 0xc7,
    ];

    let mut header = header_original.clone();

    ipv4_header_checksum(&mut header)?;

    assert_eq!(header, header_original);

    Ok(())
}

#[test]
fn header_checksum_test2() -> Result<(), &'static str> {

    let header_original: Vec<u8> = vec![
        0x45, 0x00, 0x00, 0x54, 0x9f, 0xd4, 0x40, 0x00, 0x40, 0x01, 0x81, 0x80, 0xc0, 0xa8, 0x4c, 0x02,
        0xc0, 0xa8, 0x4c, 0x01,
    ];

    let mut header = header_original.clone();

    ipv4_header_checksum(&mut header)?;

    assert_eq!(header, header_original);

    Ok(())
}