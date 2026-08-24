use crate::schema::itchformat::{AddOrderMessage, ItchMessage};

// 1. Force the compiler to pack the struct exactly like the NASDAQ ITCH 5.0 binary network spec
#[repr(packed)]
#[allow(dead_code)]
struct RawAddOrder {
    message_type: u8,            // Offset 0 (1 byte)
    stock_locate: u16,           // Offset 1 (2 bytes)
    tracking_number: u16,        // Offset 3 (2 bytes)
    timestamp: [u8; 6],          // Offset 5 (6 bytes)
    order_reference_number: u64, // Offset 11 (8 bytes)
    buy_sell_indicator: u8,      // Offset 19 (1 byte)
    shares: u32,                 // Offset 20 (4 bytes)
    stock: [u8; 8],              // Offset 24 (8 bytes)
    price: u32,                  // Offset 32 (4 bytes)
}

// 2. Accept and return the lifetime parameter '<'a>' to tie it to your main file buffer
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety check: Ensure the remaining data can safely fit the 36-byte AddOrder message packet
    if pos + 36 > data.len() {
        panic!(
            "Malformed ITCH packet: Buffer overflow while parsing AddOrder at position {}",
            pos
        );
    }

    // SAFETY: Zero-copy pointer cast is safe because:
    // 1. Bounds check above ensures data[pos..pos+36] is valid
    // 2. RawAddOrder is #[repr(packed)] matching ITCH binary layout (36 bytes)
    // 3. We only read from the memory, never write
    // 4. The lifetime 'a ensures data remains valid for the returned reference
    // 5. Pointer alignment is safe because u8 slices have no alignment requirements
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::itchformat::ItchMessage;

    #[test]
    fn test_parse_add_order() {
        // Construct a valid AddOrder message (36 bytes)
        // Message type: 'A' (0x41)
        // Stock locate: 1000 (0x03E8)
        // Tracking number: 2000 (0x07D0)
        // Timestamp: [0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
        // Order reference number: 123456789 (0x075BCD15)
        // Buy/Sell indicator: 'B' (0x42)
        // Shares: 500 (0x000001F4)
        // Stock: "AAPL    " (8 bytes)
        // Price: 15000 (0x00003A98) - representing $150.00
        let data: [u8; 36] = [
            0x41, // message_type
            0x03, 0xE8, // stock_locate (1000 in big-endian)
            0x07, 0xD0, // tracking_number (2000 in big-endian)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // timestamp
            0x00, 0x00, 0x00, 0x00, 0x07, 0x5B, 0xCD,
            0x15, // order_reference_number (123456789 in big-endian)
            0x42, // buy_sell_indicator ('B')
            0x00, 0x00, 0x01, 0xF4, // shares (500 in big-endian)
            0x41, 0x41, 0x50, 0x4C, 0x20, 0x20, 0x20, 0x20, // stock ("AAPL    ")
            0x00, 0x00, 0x3A, 0x98, // price (15000 in big-endian)
        ];

        let (bytes_consumed, msg) = parse_at(&data, 0);

        assert_eq!(bytes_consumed, 36);

        match msg {
            ItchMessage::AddOrder(add_order) => {
                assert_eq!(add_order.stock_locate, 1000);
                assert_eq!(add_order.tracking_number, 2000);
                assert_eq!(add_order.timestamp, 1);
                assert_eq!(add_order.order_reference_number, 123456789);
                assert_eq!(add_order.buy_sell_indicator, 0x42);
                assert_eq!(add_order.shares, 500);
                assert_eq!(add_order.stock, *b"AAPL    ");
                assert_eq!(add_order.price, 15000);
            }
            _ => panic!("Expected AddOrder message"),
        }
    }

    #[test]
    fn test_parse_add_order_buffer_overflow() {
        // Test that buffer overflow is detected
        let data: [u8; 30] = [0; 30]; // Less than 36 bytes

        let result = std::panic::catch_unwind(|| {
            parse_at(&data, 0);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_add_order_sell_indicator() {
        // Test with sell indicator
        let mut data: [u8; 36] = [0; 36];
        data[0] = 0x41; // message_type
        data[1] = 0x00;
        data[2] = 0x01; // stock_locate
        data[3] = 0x00;
        data[4] = 0x02; // tracking_number
        data[5] = 0x00;
        data[6] = 0x00;
        data[7] = 0x00;
        data[8] = 0x00;
        data[9] = 0x00;
        data[10] = 0x01; // timestamp
        data[11] = 0x00;
        data[12] = 0x00;
        data[13] = 0x00;
        data[14] = 0x00;
        data[15] = 0x00;
        data[16] = 0x00;
        data[17] = 0x00;
        data[18] = 0x01; // order_reference_number
        data[19] = 0x53; // buy_sell_indicator ('S' for sell)
        data[20] = 0x00;
        data[21] = 0x00;
        data[22] = 0x00;
        data[23] = 0x01; // shares
        data[24] = 0x54;
        data[25] = 0x45;
        data[26] = 0x53;
        data[27] = 0x54;
        data[28] = 0x20;
        data[29] = 0x20;
        data[30] = 0x20;
        data[31] = 0x20; // stock ("TEST    ")
        data[32] = 0x00;
        data[33] = 0x00;
        data[34] = 0x00;
        data[35] = 0x01; // price

        let (bytes_consumed, msg) = parse_at(&data, 0);

        assert_eq!(bytes_consumed, 36);

        match msg {
            ItchMessage::AddOrder(add_order) => {
                assert_eq!(add_order.buy_sell_indicator, 0x53); // 'S'
                assert_eq!(add_order.stock, *b"TEST    ");
            }
            _ => panic!("Expected AddOrder message"),
        }
    }
}
