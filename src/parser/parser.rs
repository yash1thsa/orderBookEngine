use crate::schema::itchformat::{ItchMessage, UnknownMessage};

use super::{
    add_order,
    add_order_mpid,
    order_executed_with_price,
    order_cancel,
    order_executed,
    order_delete,
    order_replace,
    cross_trade,
    stock_trading_action,
    trade,
    stock_directory,
    order_priority_update_y,
    net_order_imbalance_indicator,
    market_participant_position,
    system_event
};

/// ITCH protocol message types
#[derive(Clone, Copy, Debug)]
pub enum MessageType {
    SystemEvent,
    StockDirectory,
    AddOrder,
    OrderExecuted,
    OrderCancel,
    AddOrderMPID,
    OrderExecutedWithPrice,
    CrossTrade,
    Trade,
    OrderDelete,
    OrderReplace,
    StockTradingAction,
    OrderPriorityUpdateY,
    NetOrderImbalanceIndicator,
    MarketParticipantPosition,
    Unknown,
}

impl From<u8> for MessageType {
    fn from(b: u8) -> Self {
        match b {
            b'S' => MessageType::SystemEvent,
            b'R' => MessageType::StockDirectory,
            b'A' => MessageType::AddOrder,
            b'E' => MessageType::OrderExecuted,
            b'X' => MessageType::OrderCancel,
            b'F' => MessageType::AddOrderMPID,
            b'C' => MessageType::OrderExecutedWithPrice,
            b'Q' => MessageType::CrossTrade,
            b'P' => MessageType::Trade,
            b'D' => MessageType::OrderDelete,
            b'U' => MessageType::OrderReplace,
            b'H' => MessageType::StockTradingAction,
            b'Y' => MessageType::OrderPriorityUpdateY,
            b'I' => MessageType::NetOrderImbalanceIndicator,
            b'L' => MessageType::MarketParticipantPosition,
            _ => MessageType::Unknown,
        }
    }
}

/// ITCH protocol parser for NASDAQ order book data
///
/// This parser processes ITCH (Integrated Trading and Clearing House) protocol messages
/// using zero-copy techniques for high performance.
///
/// # Example
///
/// ```no_run
/// use orderBookEngine::parser::L3Parser;
///
/// // Parse ITCH data
/// let data = std::fs::read("data.itch")?;
/// let mut parser = L3Parser::new(&data);
///
/// while let Some(msg) = parser.parse_next() {
///     // Process message
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct L3Parser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> L3Parser<'a> {
    /// Creates a new parser from a byte slice
    ///
    /// # Arguments
    ///
    /// * `data` - The ITCH protocol data to parse
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Returns the current parsing position in bytes
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Parses all messages in the data and returns them as a vector
    ///
    /// This is a convenience method that consumes all messages at once.
    pub fn parse_all(&mut self) -> Vec<ItchMessage<'a>> {
        let mut out = Vec::new();

        while let Some(msg) = self.parse_next() {
            out.push(msg);
        }

        out
    }

    /// Parses the next message from the data
    ///
    /// Returns `None` when no more messages are available.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use orderBookEngine::parser::L3Parser;
    ///
    /// let data = std::fs::read("data.itch")?;
    /// let mut parser = L3Parser::new(&data);
    ///
    /// while let Some(msg) = parser.parse_next() {
    ///     // Process individual message
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse_next(&mut self) -> Option<ItchMessage<'a>> {
        // Need at least 2-byte length + 1-byte type
        if self.pos + 3 > self.data.len() {
            return None;
        }

        // Length of ITCH message (does NOT include the 2-byte length field)
        let msg_len = u16::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
        ]) as usize;

        // Ensure the full message is available
        if self.pos + 2 + msg_len > self.data.len() {
            return None;
        }

        // Start of the ITCH message (after the length prefix)
        let msg_start = self.pos + 2;

        let msg_type = MessageType::from(self.data[msg_start]);

        // IMPORTANT:
        // Existing parse_at() functions expect the message type byte
        // to be at data[pos], so we pass msg_start.
        // NOTE: Make sure your sub-modules (add_order, trade, etc.) are also refactored
        // to return (usize, ItchMessage<'a>)
        let (_, msg) = match msg_type {
            MessageType::SystemEvent => {
                system_event::parse_at(self.data, msg_start)
            }

            MessageType::StockDirectory => {
                stock_directory::parse_at(self.data, msg_start)
            }

            MessageType::AddOrder => {
                add_order::parse_at(self.data, msg_start)
            }

            MessageType::OrderExecuted => {
                order_executed::parse_at(self.data, msg_start)
            }

            MessageType::OrderCancel => {
                order_cancel::parse_at(self.data, msg_start)
            }
            MessageType::AddOrderMPID => {
                add_order_mpid::parse_at(self.data, msg_start)
            }
            MessageType::OrderExecutedWithPrice => {
                order_executed_with_price::parse_at(self.data, msg_start)
            }
            MessageType::CrossTrade => {
                cross_trade::parse_at(self.data, msg_start)
            }
            MessageType::Trade => {
                trade::parse_at(self.data, msg_start)
            }
            MessageType::OrderDelete => {
                order_delete::parse_at(self.data, msg_start)
            }
            MessageType::OrderReplace => {
                order_replace::parse_at(self.data, msg_start)
            }
            MessageType::StockTradingAction => {
                stock_trading_action::parse_at(self.data, msg_start)
            }
            MessageType::OrderPriorityUpdateY => {
                order_priority_update_y::parse_at(self.data, msg_start)
            }
            MessageType::NetOrderImbalanceIndicator => {
                net_order_imbalance_indicator::parse_at(self.data, msg_start)
            }
            MessageType::MarketParticipantPosition => {
                market_participant_position::parse_at(self.data, msg_start)
            }

            MessageType::Unknown => (
                msg_len,
                ItchMessage::Unknown(UnknownMessage {
                    message_type: self.data[msg_start],
                    body: &self.data[msg_start..msg_start + msg_len],
                }),
            ),
        };

        // Advance using the length prefix from the file.
        self.pos += msg_len + 2;

        Some(msg)
    }
}