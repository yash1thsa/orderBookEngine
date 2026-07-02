use crate::schema::itchformat::{ItchMessage, StockDirectoryMessage};

// 1. Force the compiler to pack the struct matching the 26-byte StockDirectory spec
#[repr(packed)]
struct RawStockDirectory {
    message_type: u8,         // Offset 0 (1 byte)
    stock_locate: u16,        // Offset 1 (2 bytes)
    tracking_number: u16,     // Offset 3 (2 bytes)
    timestamp: [u8; 6],       // Offset 5 (6 bytes)
    symbol: [u8; 8],          // Offset 11 (8 bytes) - Stock Ticker Symbol
    market_category: u8,      // Offset 19 (1 byte)
    financial_status: u8,     // Offset 20 (1 byte)
    round_lot_size: u32,      // Offset 21 (4 bytes)
    round_lots_only: u8,      // Offset 25 (1 byte)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 26 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing StockDirectory at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Map our layout straight onto the raw RAM byte address
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawStockDirectory) };

    // 4. Extract fields directly and flip Big-Endian format to CPU native format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let market_category = raw.market_category;
    let financial_status = raw.financial_status;
    let round_lot_size = u32::from_be(raw.round_lot_size);
    let round_lots_only = raw.round_lots_only;

    // Fixed arrays match raw sequential layout byte segments directly
    let symbol = raw.symbol;

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

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