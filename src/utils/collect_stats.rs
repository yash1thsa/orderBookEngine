use std::collections::HashMap;
use crate::schema::itchformat::ItchMessage;

pub struct StatsCollector {
    unknown_streak: usize,
    max_unknown_streak: usize,
    timestamp_regressions: usize,
    unknown_count: usize,
    unknown_types: HashMap<u8, usize>,
    last_timestamp: Option<u64>,
    total_messages: usize,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            unknown_streak: 0,
            max_unknown_streak: 0,
            timestamp_regressions: 0,
            unknown_count: 0,
            unknown_types: HashMap::new(),
            last_timestamp: None,
            total_messages: 0,
        }
    }

    pub fn process_message(&mut self, msg: &ItchMessage) {
        self.total_messages += 1;

        // Track unknown messages
        let name = msg.name();
        if name == "Unknown" {
            self.unknown_streak += 1;
            self.max_unknown_streak = self.max_unknown_streak.max(self.unknown_streak);
            self.unknown_count += 1;

            if let ItchMessage::Unknown(unknown_msg) = msg {
                *self.unknown_types.entry(unknown_msg.message_type).or_insert(0) += 1;
            }

            if self.unknown_streak > 1000 {
                eprintln!("WARNING: large unknown streak detected");
            }
        } else {
            self.unknown_streak = 0;
        }

        // Check timestamp monotonicity
        if let Some(ts) = msg.timestamp() {
            if let Some(prev) = self.last_timestamp {
                if ts < prev {
                    self.timestamp_regressions += 1;
                }
            }
            self.last_timestamp = Some(ts);
        }
    }

    pub fn report(&self) {
        println!("\nParsed {} messages", self.total_messages);

        println!("\nIntegrity report:");
        println!("  max_unknown_streak     : {}", self.max_unknown_streak);
        println!("  timestamp_regressions  : {}", self.timestamp_regressions);
        println!("  unknown_count          : {}", self.unknown_count);

        println!("\nUnknown message types:");
        let mut unknown_type_vec: Vec<_> = self.unknown_types.iter().collect();
        unknown_type_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
        for (msg_type, count) in unknown_type_vec {
            println!("  '{}' (0x{:02x}): {}", *msg_type as char, msg_type, count);
        }

        println!("\nTotal messages processed: {}", self.total_messages);
    }

    pub fn total_messages(&self) -> usize {
        self.total_messages
    }
}
