use crate::schema::itchformat::{ItchMessage, OrderPriorityUpdateYMessage};
use super::common::parse_timestamp;

pub fn parse_at(data: &[u8], pos: usize) -> (usize, ItchMessage) {
    let end = pos + 20;

    // SAFETY CHECK: prevent out-of-bounds panic
    if end > data.len() {
        return (
            0,
            ItchMessage::Unknown(crate::schema::itchformat::UnknownMessage {
                message_type: b'Y',
                body: vec![],
            }),
        );
    }

    let b = &data[pos..end];

    let stock_locate = u16::from_be_bytes([b[1], b[2]]);
    let tracking_number = u32::from_be_bytes([b[3], b[4], b[5], b[6]]);

    let timestamp = parse_timestamp(&b[7..13]);

    // Remaining bytes = opaque control payload
    let payload: [u8; 7] = b[13..20]
        .try_into()
        .unwrap_or([0u8; 7]);

    (
        20,
        ItchMessage::OrderPriorityUpdateY(OrderPriorityUpdateYMessage {
            stock_locate,
            tracking_number,
            timestamp,
            payload,
        }),
    )
}