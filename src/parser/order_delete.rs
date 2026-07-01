use crate::schema::itchformat::{ItchMessage, OrderDeleteMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let order_reference_number = u64::from_be_bytes(b[11..19].try_into().unwrap());

    (
        19,
        ItchMessage::OrderDelete(OrderDeleteMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
        }),
    )
}