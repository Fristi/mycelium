pub fn l2cap_att(att_pdu: &[u8]) -> Vec<u8> {
    let len = att_pdu.len() as u16;
    let mut buf = Vec::with_capacity(4 + att_pdu.len());
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&[0x04, 0x00]);
    buf.extend_from_slice(att_pdu);
    buf
}

pub fn acl_packet(handle: u16, l2cap: &[u8]) -> Vec<u8> {
    let flags = handle & 0x0FFF;
    let header = (flags | (0b00 << 12)) as u16;
    let mut buf = Vec::with_capacity(4 + l2cap.len());
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(&(l2cap.len() as u16).to_le_bytes());
    buf.extend_from_slice(l2cap);
    buf
}

pub fn att_read_request(handle: u16) -> Vec<u8> {
    let mut pdu = vec![0x0A];
    pdu.extend_from_slice(&handle.to_le_bytes());
    pdu
}

pub fn att_write_request(handle: u16, value: &[u8]) -> Vec<u8> {
    let mut pdu = vec![0x12];
    pdu.extend_from_slice(&handle.to_le_bytes());
    pdu.extend_from_slice(value);
    pdu
}

pub fn parse_att_read_response(payload: &[u8]) -> Option<Vec<u8>> {
    if payload.len() < 5 {
        return None;
    }
    let l2cap_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
    if payload.len() < 4 + l2cap_len {
        return None;
    }
    let att = &payload[4..4 + l2cap_len];
    if att.first() == Some(&0x0B) {
        return Some(att[1..].to_vec());
    }
    None
}
