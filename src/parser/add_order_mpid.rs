use crate::schema::itchformat::{ItchMessage, AddOrderMPIDMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let order_reference_number = u64::from_be_bytes(b[11..19].try_into().unwrap());
    let buy_sell_indicator = b[19];
    let shares = u32::from_be_bytes([b[20], b[21], b[22], b[23]]);
    let stock: [u8; 8] = b[24..32].try_into().unwrap();
    let price = u32::from_be_bytes([b[32], b[33], b[34], b[35]]);
    let attribution: [u8; 4] = b[36..40].try_into().unwrap();

    (
        40,
        ItchMessage::AddOrderMPID(crate::schema::itchformat::AddOrderMPIDMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
            buy_sell_indicator,
            shares,
            stock,
            price,
            attribution,
        }),
    )
}