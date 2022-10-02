
pub fn get_2_starting_at(data: &[u8], idx: usize) -> Option<[u8; 2]> {
    Some([
        *data.get(idx+0)?,
        *data.get(idx+1)?,
    ])
}

pub fn get_4_starting_at(data: &[u8], idx: usize) -> Option<[u8; 4]> {
    Some([
        *data.get(idx+0)?,
        *data.get(idx+1)?,
        *data.get(idx+2)?,
        *data.get(idx+3)?,
    ])
}

pub fn get_6_starting_at(data: &[u8], idx: usize) -> Option<[u8; 6]> {
    Some([
        *data.get(idx+0)?,
        *data.get(idx+1)?,
        *data.get(idx+2)?,
        *data.get(idx+3)?,
        *data.get(idx+4)?,
        *data.get(idx+5)?,
    ])
}

