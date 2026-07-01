use crate::schema::itchformat::{ItchMessage, StockDirectoryMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let mut symbol = [0u8; 8];
    symbol.copy_from_slice(&b[11..19]);

    let market_category = b[19];
    let financial_status = b[20];

    let round_lot_size =
        u32::from_be_bytes([b[21], b[22], b[23], b[24]]);

    let round_lots_only = b[25];

    (
        26,
        ItchMessage::StockDirectory(StockDirectoryMessage {
            stock_locate,
            tracking_number,
            timestamp,
            symbol,
            market_category,
            financial_status,
            round_lot_size,
            round_lots_only,
        }),
    )
}