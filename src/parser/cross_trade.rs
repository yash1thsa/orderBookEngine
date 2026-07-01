use crate::schema::itchformat::{ItchMessage, CrossTradeMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let shares = u64::from_be_bytes(b[11..19].try_into().unwrap());
    let stock: [u8; 8] = b[19..27].try_into().unwrap();
    let cross_price = u32::from_be_bytes([b[27], b[28], b[29], b[30]]);
    let match_number = u64::from_be_bytes(b[31..39].try_into().unwrap());
    let cross_type = b[39];

    (
        40,
        ItchMessage::CrossTrade(CrossTradeMessage {
            stock_locate,
            tracking_number,
            timestamp,
            shares,
            stock,
            cross_price,
            match_number,
            cross_type,
        }),
    )
}