use crate::schema::itchformat::{ItchMessage, OrderReplaceMessage};

// 1. Force the compiler to pack the struct matching the 35-byte OrderReplace spec
#[repr(packed)]
struct RawOrderReplace {
    message_type: u8,                    // Offset 0 (1 byte)
    stock_locate: u16,                   // Offset 1 (2 bytes)
    tracking_number: u16,                // Offset 3 (2 bytes)
    timestamp: [u8; 6],                  // Offset 5 (6 bytes)
    original_order_reference_number: u64, // Offset 11 (8 bytes)
    new_order_reference_number: u64,      // Offset 19 (8 bytes)
    shares: u32,                         // Offset 27 (4 bytes)
    price: u32,                          // Offset 31 (4 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 35 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing OrderReplace at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Read directly from raw memory addresses with zero allocation
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawOrderReplace) };

    // 4. Extract data fields directly and flip network Big-Endian format to CPU native format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let original_order_reference_number = u64::from_be(raw.original_order_reference_number);
    let new_order_reference_number = u64::from_be(raw.new_order_reference_number);
    let shares = u32::from_be(raw.shares);
    let price = u32::from_be(raw.price);

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        35,
        ItchMessage::OrderReplace(OrderReplaceMessage {
            stock_locate,
            tracking_number,
            timestamp,
            original_order_reference_number,
            new_order_reference_number,
            shares,
            price,
        }),
    )
}