use crate::schema::itchformat::{ItchMessage, OrderExecutedWithPriceMessage};

// 1. Force the compiler to pack the struct matching the 36-byte OrderExecutedWithPrice spec
#[repr(packed)]
struct RawOrderExecutedWithPrice {
    message_type: u8,            // Offset 0 (1 byte)
    stock_locate: u16,           // Offset 1 (2 bytes)
    tracking_number: u16,        // Offset 3 (2 bytes)
    timestamp: [u8; 6],          // Offset 5 (6 bytes)
    order_reference_number: u64, // Offset 11 (8 bytes)
    executed_shares: u32,        // Offset 19 (4 bytes)
    match_number: u64,           // Offset 23 (8 bytes)
    printable: u8,               // Offset 31 (1 byte)
    execution_price: u32,        // Offset 32 (4 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 36 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing OrderExecutedWithPrice at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Map our layout straight onto the raw RAM byte address
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawOrderExecutedWithPrice) };

    // 4. Extract fields directly and flip Big-Endian format to CPU native format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let order_reference_number = u64::from_be(raw.order_reference_number);
    let executed_shares = u32::from_be(raw.executed_shares);
    let match_number = u64::from_be(raw.match_number);
    let printable = raw.printable;
    let execution_price = u32::from_be(raw.execution_price);

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        36,
        ItchMessage::OrderExecutedWithPrice(OrderExecutedWithPriceMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
            executed_shares,
            match_number,
            printable,
            execution_price,
        }),
    )
}