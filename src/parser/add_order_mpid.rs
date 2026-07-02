use crate::schema::itchformat::{ItchMessage, AddOrderMPIDMessage};

// 1. Force the compiler to pack the struct matching the 40-byte AddOrderMPID spec
#[repr(packed)]
struct RawAddOrderMPID {
    message_type: u8,            // Offset 0 (1 byte)
    stock_locate: u16,           // Offset 1 (2 bytes)
    tracking_number: u16,        // Offset 3 (2 bytes)
    timestamp: [u8; 6],          // Offset 5 (6 bytes)
    order_reference_number: u64,  // Offset 11 (8 bytes)
    buy_sell_indicator: u8,      // Offset 19 (1 byte)
    shares: u32,                 // Offset 20 (4 bytes)
    stock: [u8; 8],              // Offset 24 (8 bytes)
    price: u32,                  // Offset 32 (4 bytes)
    attribution: [u8; 4],        // Offset 36 (4 bytes) - Market Participant Identifier (MPID)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy reference linking
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 40 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing AddOrderMPID at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Map our layout straight onto the raw memory address
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawAddOrderMPID) };

    // 4. Extract data directly from the address coordinates and flip Big-Endian bytes
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let order_reference_number = u64::from_be(raw.order_reference_number);
    let buy_sell_indicator = raw.buy_sell_indicator;
    let shares = u32::from_be(raw.shares);
    let price = u32::from_be(raw.price);

    // Fixed arrays copy directly across registers without endianness modifications
    let stock = raw.stock;
    let attribution = raw.attribution;

    // Optimized 6-byte inline bit-shift logic for timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        40,
        ItchMessage::AddOrderMPID(AddOrderMPIDMessage {
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