use crate::schema::itchformat::{ItchMessage, NetOrderImbalanceIndicatorMessage};

// 1. Force the compiler to pack the struct matching the 43-byte NOII spec layout
#[repr(packed)]
struct RawNetOrderImbalanceIndicator {
    message_type: u8,               // Offset 0 (1 byte)
    stock_locate: u16,              // Offset 1 (2 bytes)
    tracking_number: u16,           // Offset 3 (2 bytes)
    timestamp: [u8; 6],             // Offset 5 (6 bytes)
    paired_shares: u64,             // Offset 11 (8 bytes)
    imbalance_shares: u64,          // Offset 19 (8 bytes)
    current_reference_price: u32,   // Offset 27 (4 bytes)
    buy_sell_indicator: u8,         // Offset 31 (1 byte) -> Kept as padding/placeholder to preserve offsets
    cross_type: u8,                 // Offset 32 (1 byte)
    price_variation_indicator: u8,  // Offset 33 (1 byte)
    imbalance_direction: u8,        // Offset 34 (1 byte)
    stock: [u8; 8],                 // Offset 35 (8 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 43 > data.len() {
        panic!("Malformed ITCH packet: Buffer overflow while parsing NetOrderImbalanceIndicator at position {}", pos);
    }

    // 3. ZERO-COPY POINTER CAST: Access memory coordinates directly with zero allocation
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawNetOrderImbalanceIndicator) };

    // 4. Extract data directly from addresses and flip network Big-Endian format to CPU format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let paired_shares = u64::from_be(raw.paired_shares);
    let imbalance_shares = u64::from_be(raw.imbalance_shares);
    let current_reference_price = u32::from_be(raw.current_reference_price);

    let cross_type = raw.cross_type;
    let price_variation_indicator = raw.price_variation_indicator;
    let imbalance_direction = raw.imbalance_direction;

    // Static arrays match direct memory block layout coordinates
    let stock = raw.stock;

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp parsing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        43,
        ItchMessage::NetOrderImbalanceIndicator(NetOrderImbalanceIndicatorMessage {
            stock_locate,
            tracking_number,
            timestamp,
            paired_shares,
            imbalance_shares,
            imbalance_direction,
            stock,
            far_price: 0,   // Kept as 0 to maintain your exact struct signature
            near_price: 0,  // Kept as 0 to maintain your exact struct signature
            current_reference_price,
            cross_type,
            price_variation_indicator,
        }),
    )
}