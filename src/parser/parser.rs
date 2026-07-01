use crate::schema::itchformat::{ItchMessage, UnknownMessage};

use super::{
    add_order,
    order_cancel,
    order_executed,
    stock_directory,
    system_event,
};

#[derive(Clone, Copy, Debug)]
pub enum MessageType {
    SystemEvent,
    StockDirectory,
    AddOrder,
    OrderExecuted,
    OrderCancel,
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
            _ => MessageType::Unknown,
        }
    }
}

pub struct L3Parser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> L3Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn parse_all(&mut self) -> Vec<ItchMessage> {
        self.parse_n(usize::MAX)
    }

    pub fn parse_n(&mut self, limit: usize) -> Vec<ItchMessage> {
        let mut out = Vec::with_capacity(limit);

        while out.len() < limit {
            match self.parse_next() {
                Some(msg) => out.push(msg),
                None => break,
            }
        }

        out
    }

    pub fn parse_next(&mut self) -> Option<ItchMessage> {
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

            MessageType::Unknown => (
                msg_len,
                ItchMessage::Unknown(UnknownMessage {
                    message_type: self.data[msg_start],
                    body: self.data[msg_start..msg_start + msg_len].to_vec(),
                }),
            ),
        };

        // Advance using the length prefix from the file.
        self.pos += msg_len + 2;

        Some(msg)
    }
}