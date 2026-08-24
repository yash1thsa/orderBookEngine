use orderBookEngine::parser::L3Parser;
use orderBookEngine::schema::itchformat::ItchMessage;

#[test]
fn test_parser_integration() {
    // Create a minimal ITCH-like byte sequence with multiple message types
    // Format: [2-byte length][1-byte message type][message body]
    
    let mut data = Vec::new();
    
    // SystemEvent message (12 bytes total)
    // Length: 12, Type: 'S' (0x53)
    data.extend_from_slice(&[0x00, 0x0C]); // length (12 bytes, not including length field)
    data.push(0x53); // message type 'S'
    data.extend_from_slice(&[0x00, 0x01]); // stock_locate (1)
    data.extend_from_slice(&[0x00, 0x02]); // tracking_number (2)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x01]); // timestamp (1)
    data.push(0x4F); // event_code ('O')
    
    // AddOrder message (36 bytes total)
    // Length: 36, Type: 'A' (0x41)
    data.extend_from_slice(&[0x00, 0x24]); // length (36 bytes)
    data.push(0x41); // message type 'A'
    data.extend_from_slice(&[0x00, 0x03]); // stock_locate (3)
    data.extend_from_slice(&[0x00, 0x04]); // tracking_number (4)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x02]); // timestamp (2)
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05]); // order_ref (5)
    data.push(0x42); // buy_sell_indicator ('B')
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x64]); // shares (100)
    data.extend_from_slice(b"TEST    "); // stock
    data.extend_from_slice(&[0x00, 0x00, 0x27, 0x10]); // price (10000)
    
    let mut parser = L3Parser::new(&data);
    
    // Parse first message (SystemEvent)
    let msg1 = parser.parse_next();
    assert!(msg1.is_some());
    
    match msg1.unwrap() {
        ItchMessage::SystemEvent(sys_event) => {
            assert_eq!(sys_event.stock_locate, 1);
            assert_eq!(sys_event.tracking_number, 2);
            assert_eq!(sys_event.timestamp, 1);
            assert_eq!(sys_event.event_code, 0x4F);
        }
        _ => panic!("Expected SystemEvent"),
    }
    
    // Parse second message (AddOrder)
    let msg2 = parser.parse_next();
    assert!(msg2.is_some());
    
    match msg2.unwrap() {
        ItchMessage::AddOrder(add_order) => {
            assert_eq!(add_order.stock_locate, 3);
            assert_eq!(add_order.tracking_number, 4);
            assert_eq!(add_order.timestamp, 2);
            assert_eq!(add_order.order_reference_number, 5);
            assert_eq!(add_order.buy_sell_indicator, 0x42);
            assert_eq!(add_order.shares, 100);
            assert_eq!(add_order.stock, *b"TEST    ");
            assert_eq!(add_order.price, 10000);
        }
        _ => panic!("Expected AddOrder"),
    }
    
    // No more messages
    let msg3 = parser.parse_next();
    assert!(msg3.is_none());
}

#[test]
fn test_parser_position_tracking() {
    // Test that parser correctly tracks position through messages
    let data: Vec<u8> = vec![
        0x00, 0x0C, // length (12)
        0x53,       // type 'S'
        0x00, 0x01, // stock_locate
        0x00, 0x02, // tracking_number
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // timestamp
        0x4F,       // event_code
    ];
    
    let mut parser = L3Parser::new(&data);
    
    // Initial position should be 0
    assert_eq!(parser.position(), 0);
    
    // After parsing, position should advance
    parser.parse_next();
    assert_eq!(parser.position(), 14); // 2 (length) + 12 (message)
    
    // No more messages, position should be at end
    assert!(parser.parse_next().is_none());
}

#[test]
fn test_parser_empty_data() {
    let data: &[u8] = &[];
    let mut parser = L3Parser::new(data);
    
    assert!(parser.parse_next().is_none());
}

#[test]
fn test_parser_incomplete_message() {
    // Data with incomplete message (length says 12 bytes but only 5 available)
    let data: Vec<u8> = vec![
        0x00, 0x0C, // length (12)
        0x53,       // type 'S'
        0x00, 0x01, // stock_locate
    ];
    
    let mut parser = L3Parser::new(&data);
    
    // Should return None for incomplete message
    assert!(parser.parse_next().is_none());
}

#[test]
fn test_parse_all() {
    let mut data = Vec::new();
    
    // Add two SystemEvent messages
    for i in 0..2u64 {
        data.extend_from_slice(&[0x00, 0x0C]); // length
        data.push(0x53); // type 'S'
        data.extend_from_slice(&[0x00, 0x01]); // stock_locate
        data.extend_from_slice(&[0x00, 0x02]); // tracking_number
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, i as u8]); // timestamp
        data.push(0x4F); // event_code
    }
    
    let mut parser = L3Parser::new(&data);
    let messages = parser.parse_all();
    
    assert_eq!(messages.len(), 2);
    
    match &messages[0] {
        ItchMessage::SystemEvent(sys_event) => {
            assert_eq!(sys_event.timestamp, 0);
        }
        _ => panic!("Expected SystemEvent"),
    }
    
    match &messages[1] {
        ItchMessage::SystemEvent(sys_event) => {
            assert_eq!(sys_event.timestamp, 1);
        }
        _ => panic!("Expected SystemEvent"),
    }
}
