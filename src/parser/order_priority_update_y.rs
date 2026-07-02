use crate::schema::itchformat::{ItchMessage, OrderPriorityUpdateYMessage, UnknownMessage};

// 1. Force the compiler to pack the struct matching the 20-byte OrderPriorityUpdateY spec layout
#[repr(packed)]
struct RawOrderPriorityUpdateY {
    message_type: u8,     // Offset 0 (1 byte)
    stock_locate: u16,    // Offset 1 (2 bytes)
    tracking_number: u32, // Offset 3 (4 bytes)
    timestamp: [u8; 6],   // Offset 7 (6 bytes)
    payload: [u8; 7],     // Offset 13 (7 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy reference linking
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // 3. ZERO-ALLOCATION SAFETY CHECK: Fallback returns a borrowed slice instead of hitting the heap
    if pos + 20 > data.len() {
        return (
            0,
            ItchMessage::Unknown(UnknownMessage {
                message_type: b'Y',
                body: &data[pos..], // Zero-copy borrow of whatever remainder exists
            }),
        );
    }

    // 4. ZERO-COPY POINTER CAST: Map our struct layout straight onto the raw memory coordinates
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawOrderPriorityUpdateY) };

    // 5. Extract data and flip network Big-Endian format to CPU native integers
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u32::from_be(raw.tracking_number);

    // Fixed arrays copy natively across registers with no endianness translation required
    let payload = raw.payload;

    // Optimized 6-byte inline bit-shift logic for zero-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

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