use crate::schema::itchformat::{ItchMessage, SystemEventMessage};

// 1. Force the compiler to pack the struct matching the 12-byte SystemEvent spec
#[repr(packed)]
#[allow(dead_code)]
struct RawSystemEvent {
    message_type: u8,     // Offset 0 (1 byte)
    stock_locate: u16,    // Offset 1 (2 bytes)
    tracking_number: u16, // Offset 3 (2 bytes)
    timestamp: [u8; 6],   // Offset 5 (6 bytes)
    event_code: u8,       // Offset 11 (1 byte)
}

// 2. Accept and return the lifetime parameter '<'a>' for zero-copy connection
pub fn parse_at<'a>(data: &'a [u8], pos: usize) -> (usize, ItchMessage<'a>) {
    // Safety boundary validation check
    if pos + 12 > data.len() {
        panic!(
            "Malformed ITCH packet: Buffer overflow while parsing SystemEvent at position {}",
            pos
        );
    }

    // SAFETY: Zero-copy pointer cast is safe because:
    // 1. Bounds check above ensures data[pos..pos+12] is valid
    // 2. RawSystemEvent is #[repr(packed)] matching ITCH binary layout (12 bytes)
    // 3. We only read from the memory, never write
    // 4. The lifetime 'a ensures data remains valid for the returned reference
    // 5. Pointer alignment is safe because u8 slices have no alignment requirements
    let raw = unsafe { &*(data.as_ptr().add(pos) as *const RawSystemEvent) };

    // 4. Extract fields directly and flip Big-Endian format to CPU native format
    let stock_locate = u16::from_be(raw.stock_locate);
    let tracking_number = u16::from_be(raw.tracking_number);
    let event_code = raw.event_code;

    // Optimized 6-byte inline bit-shift logic for low-overhead timestamp processing
    let timestamp = ((raw.timestamp[0] as u64) << 40)
        | ((raw.timestamp[1] as u64) << 32)
        | ((raw.timestamp[2] as u64) << 24)
        | ((raw.timestamp[3] as u64) << 16)
        | ((raw.timestamp[4] as u64) << 8)
        | (raw.timestamp[5] as u64);

    (
        12,
        ItchMessage::SystemEvent(SystemEventMessage {
            stock_locate,
            tracking_number,
            timestamp,
            event_code,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::itchformat::ItchMessage;

    #[test]
    fn test_parse_system_event() {
        // Construct a valid SystemEvent message (12 bytes)
        // Message type: 'S' (0x53)
        // Stock locate: 1234 (0x04D2)
        // Tracking number: 5678 (0x162E)
        // Timestamp: [0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
        // Event code: 'O' (0x4F) - Start of messages
        let data: [u8; 12] = [
            0x53, // message_type
            0x04, 0xD2, // stock_locate (1234 in big-endian)
            0x16, 0x2E, // tracking_number (5678 in big-endian)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // timestamp
            0x4F, // event_code ('O')
        ];

        let (bytes_consumed, msg) = parse_at(&data, 0);

        assert_eq!(bytes_consumed, 12);

        match msg {
            ItchMessage::SystemEvent(sys_event) => {
                assert_eq!(sys_event.stock_locate, 1234);
                assert_eq!(sys_event.tracking_number, 5678);
                assert_eq!(sys_event.timestamp, 1);
                assert_eq!(sys_event.event_code, 0x4F);
            }
            _ => panic!("Expected SystemEvent message"),
        }
    }

    #[test]
    fn test_parse_system_event_buffer_overflow() {
        // Test that buffer overflow is detected
        let data: [u8; 10] = [0; 10]; // Less than 12 bytes

        let result = std::panic::catch_unwind(|| {
            parse_at(&data, 0);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_system_event_at_offset() {
        // Test parsing at a non-zero offset
        let mut data: [u8; 20] = [0; 20];
        // Insert SystemEvent at offset 5
        data[5] = 0x53; // message_type
        data[6] = 0x00;
        data[7] = 0x01; // stock_locate
        data[8] = 0x00;
        data[9] = 0x02; // tracking_number
        data[10] = 0x00;
        data[11] = 0x00;
        data[12] = 0x00;
        data[13] = 0x00;
        data[14] = 0x00;
        data[15] = 0x01; // timestamp
        data[16] = 0x4F; // event_code

        let (bytes_consumed, msg) = parse_at(&data, 5);

        assert_eq!(bytes_consumed, 12);

        match msg {
            ItchMessage::SystemEvent(sys_event) => {
                assert_eq!(sys_event.stock_locate, 1);
                assert_eq!(sys_event.tracking_number, 2);
                assert_eq!(sys_event.timestamp, 1);
                assert_eq!(sys_event.event_code, 0x4F);
            }
            _ => panic!("Expected SystemEvent message"),
        }
    }
}
