use crate::schema::itchformat::{ItchMessage, OrderReplaceMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let original_order_reference_number = u64::from_be_bytes(b[11..19].try_into().unwrap());
    let new_order_reference_number = u64::from_be_bytes(b[19..27].try_into().unwrap());
    let shares = u32::from_be_bytes([b[27], b[28], b[29], b[30]]);
    let price = u32::from_be_bytes([b[31], b[32], b[33], b[34]]);

    (
        35,
        ItchMessage::OrderReplace(OrderReplaceMessage {
            stock_locate,
            tracking_number,
            timestamp,
            original_order_reference_number,
            new_order_reference_number,
            shares,
            price,
        }),
    )
}