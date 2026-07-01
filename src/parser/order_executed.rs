use crate::schema::itchformat::{ItchMessage, OrderExecutedMessage};

fn ts(b: &[u8]) -> u64 {
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
    let timestamp = ts(&b[5..11]);

    let order_reference_number =
        u64::from_be_bytes(b[11..19].try_into().unwrap());

    let executed_shares =
        u32::from_be_bytes([b[19], b[20], b[21], b[22]]);

    let match_number =
        u64::from_be_bytes(b[23..31].try_into().unwrap());

    (
        31,
        ItchMessage::OrderExecuted(OrderExecutedMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
            executed_shares,
            match_number,
        }),
    )
}