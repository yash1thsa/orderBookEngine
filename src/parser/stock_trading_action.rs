use crate::schema::itchformat::{ItchMessage, StockTradingActionMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let stock: [u8; 8] = b[11..19].try_into().unwrap();
    let trading_state = b[19];
    let reserved = b[20];
    let reason = b[21];

    (
        22,
        ItchMessage::StockTradingAction(StockTradingActionMessage {
            stock_locate,
            tracking_number,
            timestamp,
            stock,
            trading_state,
            reserved,
            reason,
        }),
    )
}