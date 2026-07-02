use crate::schema::itchformat::{ItchMessage, TradeMessage};

// 1. Force the compiler to pack the struct matching the 44-byte Trade spec
#[repr(packed)]
struct RawTrade {
    message_type: u8,            // Offset 0 (1 byte)
    stock_locate: u16,           // Offset 1 (2 bytes)
    tracking_number: u16,        // Offset 3 (2 bytes)
    timestamp: [u8; 6],          // Offset 5 (6 bytes)
    order_reference_number: u64, // Offset 11 (8 bytes)
    buy_sell_indicator: u8,      // Offset 19 (1 byte)
    shares: u32,                 // Offset 20 (4 bytes)
    stock: [u8; 8],              // Offset 24 (8 bytes)
    price: u32,                  // Offset 32 (4 bytes)
    match_number: u64,           // Offset 36 (8 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 44 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing Trade at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Map our layout straight onto the raw RAM byte address
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawTrade) };

    // 4. Extract fields directly and flip Big-Endian format to CPU native format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let order_reference_number = u64::from_be(raw.order_reference_number);
    let buy_sell_indicator = raw.buy_sell_indicator;
    let shares = u32::from_be(raw.shares);
    let price = u32::from_be(raw.price);
    let match_number = u64::from_be(raw.match_number);

    // Fixed arrays match raw sequential layout byte segments directly
    let stock = raw.stock;

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        44,
        ItchMessage::Trade(TradeMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
            buy_sell_indicator,
            shares,
            stock,
            price,
            match_number,
        }),
    )
}