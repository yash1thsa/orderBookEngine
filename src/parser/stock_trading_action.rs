use crate::schema::itchformat::{ItchMessage, StockTradingActionMessage};

// 1. Force the compiler to pack the struct matching the 22-byte StockTradingAction spec
#[repr(packed)]
struct RawStockTradingAction {
    message_type: u8,     // Offset 0 (1 byte)
    stock_locate: u16,    // Offset 1 (2 bytes)
    tracking_number: u16, // Offset 3 (2 bytes)
    timestamp: [u8; 6],   // Offset 5 (6 bytes)
    stock: [u8; 8],       // Offset 11 (8 bytes)
    trading_state: u8,    // Offset 19 (1 byte)
    reserved: u8,         // Offset 20 (1 byte)
    reason: u8,           // Offset 21 (1 byte)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 22 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing StockTradingAction at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Map our layout straight onto the raw RAM byte address
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawStockTradingAction) };

    // 4. Extract fields directly and flip Big-Endian format to CPU native format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let trading_state = raw.trading_state;
    let reserved = raw.reserved;
    let reason = raw.reason;

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
        22,
        ItchMessage::StockTradingAction(StockTradingActionMessage {
            stock_locate,
            tracking_number,
            timestamp,
            stock,
            trading_state,
            reserved,
            reason,
        }),
    )
}