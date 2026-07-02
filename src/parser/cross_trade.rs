use crate::schema::itchformat::{ItchMessage, CrossTradeMessage};

// 1. Force the compiler to pack the struct matching the 40-byte CrossTrade spec
#[repr(packed)]
struct RawCrossTrade {
    message_type: u8,     // Offset 0 (1 byte)
    stock_locate: u16,    // Offset 1 (2 bytes)
    tracking_number: u16, // Offset 3 (2 bytes)
    timestamp: [u8; 6],   // Offset 5 (6 bytes)
    shares: u64,          // Offset 11 (8 bytes)
    stock: [u8; 8],       // Offset 19 (8 bytes)
    cross_price: u32,     // Offset 27 (4 bytes)
    match_number: u64,    // Offset 31 (8 bytes)
    cross_type: u8,       // Offset 39 (1 byte)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary check
    if pos + 40 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing CrossTrade at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Access fields straight from memory addresses
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawCrossTrade) };

    // 4. Extract data directly from coordinates and flip network Big-Endian format to native CPU format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let shares = u64::from_be(raw.shares);
    let cross_price = u32::from_be(raw.cross_price);
    let match_number = u64::from_be(raw.match_number);
    let cross_type = raw.cross_type;

    // Static arrays map straight across without endianness modifications
    let stock = raw.stock;

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

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