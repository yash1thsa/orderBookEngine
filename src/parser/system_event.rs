use crate::schema::itchformat::{ItchMessage, SystemEventMessage};
use super::common::parse_timestamp;

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