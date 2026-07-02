use crate::schema::itchformat::{ItchMessage, NetOrderImbalanceIndicatorMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let paired_shares = u64::from_be_bytes(b[11..19].try_into().unwrap());
    let imbalance_shares = u64::from_be_bytes(b[19..27].try_into().unwrap());
    let current_reference_price = u32::from_be_bytes([b[27], b[28], b[29], b[30]]);

    let _buy_sell_indicator = b[31];
    let cross_type = b[32];
    let price_variation_indicator = b[33];
    let imbalance_direction = b[34];

    let stock: [u8; 8] = b[35..43].try_into().unwrap();

    (
        43,
        ItchMessage::NetOrderImbalanceIndicator(NetOrderImbalanceIndicatorMessage {
            stock_locate,
            tracking_number,
            timestamp,
            paired_shares,
            imbalance_shares,
            imbalance_direction,
            stock,
            far_price: 0,
            near_price: 0,
            current_reference_price,
            cross_type,
            price_variation_indicator,
        }),
    )
}
