use crate::schema::itchformat::{ItchMessage, OrderExecutedWithPriceMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let order_reference_number = u64::from_be_bytes(b[11..19].try_into().unwrap());
    let executed_shares = u32::from_be_bytes([b[19], b[20], b[21], b[22]]);
    let match_number = u64::from_be_bytes(b[23..31].try_into().unwrap());
    let printable = b[31];
    let execution_price = u32::from_be_bytes([b[32], b[33], b[34], b[35]]);

    (
        36,
        ItchMessage::OrderExecutedWithPrice(OrderExecutedWithPriceMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
            executed_shares,
            match_number,
            printable,
            execution_price,
        }),
    )
}