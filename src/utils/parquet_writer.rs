use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use arrow::array::*;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::fs::File;
use crate::schema::itchformat::*;

// ⚡ ADDED: The struct now accepts lifetime `'a` to safely hold zero-copy messages in its batch cache
pub struct ParquetWriter<'a> {
    batch_size: usize,
    output_dir: String,
    batches: HashMap<String, Vec<ItchMessage<'a>>>,
    file_counters: HashMap<String, usize>,
}

impl<'a> ParquetWriter<'a> {
    pub fn new(output_dir: String, batch_size: usize) -> Self {
        fs::create_dir_all(&output_dir).ok();
        Self {
            batch_size,
            output_dir,
            batches: HashMap::new(),
            file_counters: HashMap::new(),
        }
    }

    pub fn add_message(&mut self, msg: ItchMessage<'a>) {
        let msg_type = msg.name().to_string();
        self.batches.entry(msg_type.clone()).or_insert_with(Vec::new).push(msg);

        if let Some(batch) = self.batches.get(&msg_type) {
            if batch.len() >= self.batch_size {
                let batch_to_write = self.batches.remove(&msg_type).unwrap();
                let _ = self.write_batch(&msg_type, batch_to_write);
            }
        }
    }

    fn write_batch(&mut self, msg_type: &str, messages: Vec<ItchMessage<'a>>) {
        let counter = self.file_counters.entry(msg_type.to_string()).or_insert(0);
        let filename = format!("{}/{}_{:06}.parquet", self.output_dir, msg_type, counter);
        *counter += 1;

        match msg_type {
            "SystemEvent" => self.write_system_event(&filename, messages),
            "StockDirectory" => self.write_stock_directory(&filename, messages),
            "AddOrder" => self.write_add_order(&filename, messages),
            "OrderExecuted" => self.write_order_executed(&filename, messages),
            "OrderCancel" => self.write_order_cancel(&filename, messages),
            "AddOrderMPID" => self.write_add_order_mpid(&filename, messages),
            "OrderExecutedWithPrice" => self.write_order_executed_with_price(&filename, messages),
            "CrossTrade" => self.write_cross_trade(&filename, messages),
            "Trade" => self.write_trade(&filename, messages),
            "OrderDelete" => self.write_order_delete(&filename, messages),
            "OrderReplace" => self.write_order_replace(&filename, messages),
            "StockTradingAction" => self.write_stock_trading_action(&filename, messages),
            "OrderPriorityUpdateY" => self.write_order_priority_update_y(&filename, messages),
            "NetOrderImbalanceIndicator" => self.write_net_order_imbalance_indicator(&filename, messages),
            "MarketParticipantPosition" => self.write_market_participant_position(&filename, messages),
            _ => eprintln!("Unknown message type: {}", msg_type),
        }
    }

    fn write_system_event(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut timestamps = Vec::new();
        let mut event_codes = Vec::new();

        for msg in messages {
            if let ItchMessage::SystemEvent(m) = msg {
                timestamps.push(m.timestamp as i64);
                event_codes.push(m.event_code as i32);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
                Arc::new(Int32Array::from(event_codes)) as Arc<dyn Array>,
            ],
            vec!["timestamp", "event_code"],
        );
    }

    fn write_stock_directory(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut tracking_numbers = Vec::new();
        let mut timestamps = Vec::new();
        let mut symbols = Vec::new();

        for msg in messages {
            if let ItchMessage::StockDirectory(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                tracking_numbers.push(m.tracking_number as i32);
                timestamps.push(m.timestamp as i64);
                symbols.push(String::from_utf8_lossy(&m.symbol).to_string());
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int32Array::from(tracking_numbers)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
                Arc::new(StringArray::from(symbols)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "tracking_number", "timestamp", "symbol"],
        );
    }

    fn write_add_order(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();
        let mut prices = Vec::new();
        let mut shares = Vec::new();

        for msg in messages {
            if let ItchMessage::AddOrder(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
                prices.push(m.price as i32);
                shares.push(m.shares as i32);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
                Arc::new(Int32Array::from(prices)) as Arc<dyn Array>,
                Arc::new(Int32Array::from(shares)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp", "price", "shares"],
        );
    }

    fn write_order_executed(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();
        let mut executed_shares = Vec::new();

        for msg in messages {
            if let ItchMessage::OrderExecuted(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
                executed_shares.push(m.executed_shares as i32);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
                Arc::new(Int32Array::from(executed_shares)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp", "executed_shares"],
        );
    }

    fn write_order_cancel(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::OrderCancel(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_add_order_mpid(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::AddOrderMPID(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_order_executed_with_price(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();
        let mut prices = Vec::new();

        for msg in messages {
            if let ItchMessage::OrderExecutedWithPrice(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
                prices.push(m.execution_price as i32);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
                Arc::new(Int32Array::from(prices)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp", "price"],
        );
    }

    fn write_cross_trade(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::CrossTrade(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_trade(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::Trade(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_order_delete(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::OrderDelete(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_order_replace(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::OrderReplace(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_stock_trading_action(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::StockTradingAction(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_order_priority_update_y(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::OrderPriorityUpdateY(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_net_order_imbalance_indicator(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();
        let mut imbalance_shares = Vec::new();

        for msg in messages {
            if let ItchMessage::NetOrderImbalanceIndicator(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
                imbalance_shares.push(m.imbalance_shares as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(imbalance_shares)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp", "imbalance_shares"],
        );
    }

    fn write_market_participant_position(&self, filename: &str, messages: Vec<ItchMessage<'a>>) {
        let mut stock_locates = Vec::new();
        let mut timestamps = Vec::new();

        for msg in messages {
            if let ItchMessage::MarketParticipantPosition(m) = msg {
                stock_locates.push(m.stock_locate as i32);
                timestamps.push(m.timestamp as i64);
            }
        }

        self.write_parquet(
            filename,
            vec![
                Arc::new(Int32Array::from(stock_locates)) as Arc<dyn Array>,
                Arc::new(Int64Array::from(timestamps)) as Arc<dyn Array>,
            ],
            vec!["stock_locate", "timestamp"],
        );
    }

    fn write_parquet(&self, filename: &str, arrays: Vec<Arc<dyn Array>>, column_names: Vec<&str>) {
        let fields: Vec<Field> = column_names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let data_type = match arrays[i].data_type() {
                    DataType::Int32 => DataType::Int32,
                    DataType::Int64 => DataType::Int64,
                    DataType::Utf8 => DataType::Utf8,
                    dt => dt.clone(),
                };
                Field::new(*name, data_type, true)
            })
            .collect();

        let schema = Arc::new(Schema::new(fields));
        let batch = RecordBatch::try_new(schema.clone(), arrays).expect("Failed to create record batch");

        let file = File::create(filename).expect("Failed to create file");
        let mut writer = ArrowWriter::try_new(file, schema, None).expect("Failed to create writer");
        writer.write(&batch).expect("Failed to write batch");
        let _ = writer.into_inner();
    }

    pub fn flush_remaining(&mut self) {
        let batches_to_write: Vec<_> = self.batches.drain().collect();
        for (msg_type, messages) in batches_to_write {
            if !messages.is_empty() {
                let _ = self.write_batch(&msg_type, messages);
            }
        }
    }
}