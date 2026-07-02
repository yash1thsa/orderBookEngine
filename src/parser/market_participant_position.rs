use crate::schema::itchformat::{ItchMessage, MarketParticipantPositionMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let b = &data[pos..];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u16::from_be_bytes([b[3], b[4]]);
    let timestamp = parse_timestamp(&b[5..11]);

    let _mpid = u32::from_be_bytes([b[11], b[12], b[13], b[14]]);

    let stock: [u8; 8] = b[15..23].try_into().unwrap();

    let primary_market_maker = b[23];
    let market_maker_mode = b[24];
    let market_participant_state = b[25];

    (
        26,
        ItchMessage::MarketParticipantPosition(MarketParticipantPositionMessage {
            stock_locate,
            tracking_number,
            timestamp,
            stock,
            primary_market_maker,
            market_maker_mode,
            market_participant_state,
        }),
    )
}
