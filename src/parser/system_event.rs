use crate::schema::itchformat::{ItchMessage, SystemEventMessage};

fn parse_timestamp(b: &[u8]) -> u64 {
    ((b[0] as u64) << 40)
        | ((b[1] as u64) << 32)
        | ((b[2] as u64) << 24)
        | ((b[3] as u64) << 16)
        | ((b[4] as u64) << 8)
        | (b[5] as u64)
}

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);
    let event_code = b[11];

    (
        12,
        ItchMessage::SystemEvent(SystemEventMessage {
            stock_locate,
            tracking_number,
            timestamp,
            event_code,
        }),
    )
}