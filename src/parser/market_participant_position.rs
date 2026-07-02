use crate::schema::itchformat::{ItchMessage, MarketParticipantPositionMessage};

// 1. Force the compiler to pack the struct matching the 26-byte MarketParticipantPosition spec
#[repr(packed)]
struct RawMarketParticipantPosition {
    message_type: u8,               // Offset 0 (1 byte)
    stock_locate: u16,              // Offset 1 (2 bytes)
    tracking_number: u16,           // Offset 3 (2 bytes)
    timestamp: [u8; 6],             // Offset 5 (6 bytes)
    mpid: [u8; 4],                  // Offset 11 (4 bytes) - Market Participant ID
    stock: [u8; 8],                 // Offset 15 (8 bytes)
    primary_market_maker: u8,       // Offset 23 (1 byte)
    market_maker_mode: u8,          // Offset 24 (1 byte)
    market_participant_state: u8,    // Offset 25 (1 byte)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 26 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing MarketParticipantPosition at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Read directly from raw memory address locations
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawMarketParticipantPosition) };

    // 4. Extract fields and convert Big-Endian network format to native CPU integers
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let primary_market_maker = raw.primary_market_maker;
    let market_maker_mode = raw.market_maker_mode;
    let market_participant_state = raw.market_participant_state;

    // Fixed arrays match raw sequential layout byte segments directly
    let stock = raw.stock;

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp parsing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

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