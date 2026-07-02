use crate::schema::itchformat::{ItchMessage, AddOrderMessage};

// 1. Force the compiler to pack the struct exactly like the NASDAQ ITCH 5.0 binary network spec
#[repr(packed)]
struct RawAddOrder {
    message_type: u8,           // Offset 0 (1 byte)
    stock_locate: u16,          // Offset 1 (2 bytes)
    tracking_number: u16,       // Offset 3 (2 bytes)
    timestamp: [u8; 6],         // Offset 5 (6 bytes)
    order_reference_number: u64, // Offset 11 (8 bytes)
    buy_sell_indicator: u8,     // Offset 19 (1 byte)
    shares: u32,                // Offset 20 (4 bytes)
    stock: [u8; 8],             // Offset 24 (8 bytes)
    price: u32,                 // Offset 32 (4 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' to tie it to your main file buffer
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety check: Ensure the remaining data can safely fit the 36-byte AddOrder message packet
    if pos + 36 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing AddOrder at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Map our struct straight onto the raw RAM byte address
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawAddOrder) };

    // 4. Read directly from the reference address and flip network Big-Endian format to native CPU format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let order_reference_number = u64::from_be(raw.order_reference_number);
    let buy_sell_indicator = raw.buy_sell_indicator;
    let shares = u32::from_be(raw.shares);
    let price = u32::from_be(raw.price);

    // Arrays don't need endianness flipping, they are already sequential bytes
    let stock = raw.stock;

    // Custom inline timestamp parsing (avoids slice generation overhead)
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        36,
        ItchMessage::AddOrder(AddOrderMessage {
            stock_locate,
            tracking_number,
            timestamp,
            order_reference_number,
            buy_sell_indicator,
            shares,
            stock,
            price,
        }),
    )
}