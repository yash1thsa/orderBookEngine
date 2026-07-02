use std::fs;
use std::sync::Arc;
use arrow::array::*;
use arrow::datatypes::{Field, Schema};
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::FileWriter; // ⚡ Pure Arrow Stream writer
use std::fs::File;
use crate::schema::itchformat::*;

pub struct L3Writer {
    batch_size: usize,
    output_dir: String,
    file_counters: [usize; 15],

    // Direct flat primitive buffers
    se_timestamp: Vec<i64>, se_event_code: Vec<i32>,
    sd_stock_locate: Vec<i32>, sd_tracking_number: Vec<i32>, sd_timestamp: Vec<i64>, sd_symbol: Vec<i64>,
    ao_stock_locate: Vec<i32>, ao_timestamp: Vec<i64>, ao_price: Vec<i32>, ao_shares: Vec<i32>,
    oe_stock_locate: Vec<i32>, oe_timestamp: Vec<i64>, oe_executed_shares: Vec<i32>,
    oc_stock_locate: Vec<i32>, oc_timestamp: Vec<i64>,
    aom_stock_locate: Vec<i32>, aom_timestamp: Vec<i64>,
    oep_stock_locate: Vec<i32>, oep_timestamp: Vec<i64>, oep_price: Vec<i32>,
    ct_stock_locate: Vec<i32>, ct_timestamp: Vec<i64>,
    t_stock_locate: Vec<i32>, t_timestamp: Vec<i64>,
    od_stock_locate: Vec<i32>, od_timestamp: Vec<i64>,
    or_stock_locate: Vec<i32>, or_timestamp: Vec<i64>,
    sta_stock_locate: Vec<i32>, sta_timestamp: Vec<i64>,
    opu_stock_locate: Vec<i32>, opu_timestamp: Vec<i64>,
    noi_stock_locate: Vec<i32>, noi_timestamp: Vec<i64>, noi_imbalance_shares: Vec<i64>,
    mpp_stock_locate: Vec<i32>, mpp_timestamp: Vec<i64>,
}

impl L3Writer {
    pub fn new(output_dir: String, batch_size: usize) -> Self {
        fs::create_dir_all(&output_dir).ok();
        Self {
            batch_size,
            output_dir,
            file_counters: [0; 15],
            se_timestamp: Vec::with_capacity(batch_size), se_event_code: Vec::with_capacity(batch_size),
            sd_stock_locate: Vec::with_capacity(batch_size), sd_tracking_number: Vec::with_capacity(batch_size), sd_timestamp: Vec::with_capacity(batch_size), sd_symbol: Vec::with_capacity(batch_size),
            ao_stock_locate: Vec::with_capacity(batch_size), ao_timestamp: Vec::with_capacity(batch_size), ao_price: Vec::with_capacity(batch_size), ao_shares: Vec::with_capacity(batch_size),
            oe_stock_locate: Vec::with_capacity(batch_size), oe_timestamp: Vec::with_capacity(batch_size), oe_executed_shares: Vec::with_capacity(batch_size),
            oc_stock_locate: Vec::with_capacity(batch_size), oc_timestamp: Vec::with_capacity(batch_size),
            aom_stock_locate: Vec::with_capacity(batch_size), aom_timestamp: Vec::with_capacity(batch_size),
            oep_stock_locate: Vec::with_capacity(batch_size), oep_timestamp: Vec::with_capacity(batch_size), oep_price: Vec::with_capacity(batch_size),
            ct_stock_locate: Vec::with_capacity(batch_size), ct_timestamp: Vec::with_capacity(batch_size),
            t_stock_locate: Vec::with_capacity(batch_size), t_timestamp: Vec::with_capacity(batch_size),
            od_stock_locate: Vec::with_capacity(batch_size), od_timestamp: Vec::with_capacity(batch_size),
            or_stock_locate: Vec::with_capacity(batch_size), or_timestamp: Vec::with_capacity(batch_size),
            sta_stock_locate: Vec::with_capacity(batch_size), sta_timestamp: Vec::with_capacity(batch_size),
            opu_stock_locate: Vec::with_capacity(batch_size), opu_timestamp: Vec::with_capacity(batch_size),
            noi_stock_locate: Vec::with_capacity(batch_size), noi_timestamp: Vec::with_capacity(batch_size), noi_imbalance_shares: Vec::with_capacity(batch_size),
            mpp_stock_locate: Vec::with_capacity(batch_size), mpp_timestamp: Vec::with_capacity(batch_size),
        }
    }

    pub fn add_message(&mut self, msg: ItchMessage) {
        match msg {
            ItchMessage::SystemEvent(m) => {
                self.se_timestamp.push(m.timestamp as i64);
                self.se_event_code.push(m.event_code as i32);
                if self.se_timestamp.len() >= self.batch_size { self.flush_system_event(); }
            }
            ItchMessage::StockDirectory(m) => {
                self.sd_stock_locate.push(m.stock_locate as i32);
                self.sd_tracking_number.push(m.tracking_number as i32);
                self.sd_timestamp.push(m.timestamp as i64);
                self.sd_symbol.push(i64::from_le_bytes(m.symbol));
                if self.sd_stock_locate.len() >= self.batch_size { self.flush_stock_directory(); }
            }
            ItchMessage::AddOrder(m) => {
                self.ao_stock_locate.push(m.stock_locate as i32);
                self.ao_timestamp.push(m.timestamp as i64);
                self.ao_price.push(m.price as i32);
                self.ao_shares.push(m.shares as i32);
                if self.ao_stock_locate.len() >= self.batch_size { self.flush_add_order(); }
            }
            ItchMessage::OrderExecuted(m) => {
                self.oe_stock_locate.push(m.stock_locate as i32);
                self.oe_timestamp.push(m.timestamp as i64);
                self.oe_executed_shares.push(m.executed_shares as i32);
                if self.oe_stock_locate.len() >= self.batch_size { self.flush_order_executed(); }
            }
            ItchMessage::OrderCancel(m) => {
                self.oc_stock_locate.push(m.stock_locate as i32);
                self.oc_timestamp.push(m.timestamp as i64);
                if self.oc_stock_locate.len() >= self.batch_size { self.flush_order_cancel(); }
            }
            ItchMessage::AddOrderMPID(m) => {
                self.aom_stock_locate.push(m.stock_locate as i32);
                self.aom_timestamp.push(m.timestamp as i64);
                if self.aom_stock_locate.len() >= self.batch_size { self.flush_add_order_mpid(); }
            }
            ItchMessage::OrderExecutedWithPrice(m) => {
                self.oep_stock_locate.push(m.stock_locate as i32);
                self.oep_timestamp.push(m.timestamp as i64);
                self.oep_price.push(m.execution_price as i32);
                if self.oep_stock_locate.len() >= self.batch_size { self.flush_order_executed_with_price(); }
            }
            ItchMessage::CrossTrade(m) => {
                self.ct_stock_locate.push(m.stock_locate as i32);
                self.ct_timestamp.push(m.timestamp as i64);
                if self.ct_stock_locate.len() >= self.batch_size { self.flush_cross_trade(); }
            }
            ItchMessage::Trade(m) => {
                self.t_stock_locate.push(m.stock_locate as i32);
                self.t_timestamp.push(m.timestamp as i64);
                if self.t_stock_locate.len() >= self.batch_size { self.flush_trade(); }
            }
            ItchMessage::OrderDelete(m) => {
                self.od_stock_locate.push(m.stock_locate as i32);
                self.od_timestamp.push(m.timestamp as i64);
                if self.od_stock_locate.len() >= self.batch_size { self.flush_order_delete(); }
            }
            ItchMessage::OrderReplace(m) => {
                self.or_stock_locate.push(m.stock_locate as i32);
                self.or_timestamp.push(m.timestamp as i64);
                if self.or_stock_locate.len() >= self.batch_size { self.flush_order_replace(); }
            }
            ItchMessage::StockTradingAction(m) => {
                self.sta_stock_locate.push(m.stock_locate as i32);
                self.sta_timestamp.push(m.timestamp as i64);
                if self.sta_stock_locate.len() >= self.batch_size { self.flush_stock_trading_action(); }
            }
            ItchMessage::OrderPriorityUpdateY(m) => {
                self.opu_stock_locate.push(m.stock_locate as i32);
                self.opu_timestamp.push(m.timestamp as i64);
                if self.opu_stock_locate.len() >= self.batch_size { self.flush_order_priority_update_y(); }
            }
            ItchMessage::NetOrderImbalanceIndicator(m) => {
                self.noi_stock_locate.push(m.stock_locate as i32);
                self.noi_timestamp.push(m.timestamp as i64);
                self.noi_imbalance_shares.push(m.imbalance_shares as i64);
                if self.noi_stock_locate.len() >= self.batch_size { self.flush_net_order_imbalance_indicator(); }
            }
            ItchMessage::MarketParticipantPosition(m) => {
                self.mpp_stock_locate.push(m.stock_locate as i32);
                self.mpp_timestamp.push(m.timestamp as i64);
                if self.mpp_stock_locate.len() >= self.batch_size { self.flush_market_participant_position(); }
            }
            ItchMessage::Unknown(_) => {}
        }
    }

    fn flush_system_event(&mut self) {
        let filename = format!("{}/SystemEvent_{:06}.arrow", self.output_dir, self.file_counters[0]);
        self.file_counters[0] += 1;
        let ts = std::mem::replace(&mut self.se_timestamp, Vec::with_capacity(self.batch_size));
        let ec = std::mem::replace(&mut self.se_event_code, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int64Array::from(ts)), Arc::new(Int32Array::from(ec))], vec!["timestamp", "event_code"]);
    }

    fn flush_stock_directory(&mut self) {
        let filename = format!("{}/StockDirectory_{:06}.arrow", self.output_dir, self.file_counters[1]);
        self.file_counters[1] += 1;
        let sl = std::mem::replace(&mut self.sd_stock_locate, Vec::with_capacity(self.batch_size));
        let tn = std::mem::replace(&mut self.sd_tracking_number, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.sd_timestamp, Vec::with_capacity(self.batch_size));
        let sym = std::mem::replace(&mut self.sd_symbol, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int32Array::from(tn)), Arc::new(Int64Array::from(ts)), Arc::new(Int64Array::from(sym))], vec!["stock_locate", "tracking_number", "timestamp", "symbol_raw_i64"]);
    }

    fn flush_add_order(&mut self) {
        let filename = format!("{}/AddOrder_{:06}.arrow", self.output_dir, self.file_counters[2]);
        self.file_counters[2] += 1;
        let sl = std::mem::replace(&mut self.ao_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.ao_timestamp, Vec::with_capacity(self.batch_size));
        let pr = std::mem::replace(&mut self.ao_price, Vec::with_capacity(self.batch_size));
        let sh = std::mem::replace(&mut self.ao_shares, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts)), Arc::new(Int32Array::from(pr)), Arc::new(Int32Array::from(sh))], vec!["stock_locate", "timestamp", "price", "shares"]);
    }

    fn flush_order_executed(&mut self) {
        let filename = format!("{}/OrderExecuted_{:06}.arrow", self.output_dir, self.file_counters[3]);
        self.file_counters[3] += 1;
        let sl = std::mem::replace(&mut self.oe_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.oe_timestamp, Vec::with_capacity(self.batch_size));
        let es = std::mem::replace(&mut self.oe_executed_shares, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts)), Arc::new(Int32Array::from(es))], vec!["stock_locate", "timestamp", "executed_shares"]);
    }

    fn flush_order_cancel(&mut self) {
        let filename = format!("{}/OrderCancel_{:06}.arrow", self.output_dir, self.file_counters[4]);
        self.file_counters[4] += 1;
        let sl = std::mem::replace(&mut self.oc_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.oc_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_add_order_mpid(&mut self) {
        let filename = format!("{}/AddOrderMPID_{:06}.arrow", self.output_dir, self.file_counters[5]);
        self.file_counters[5] += 1;
        let sl = std::mem::replace(&mut self.aom_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.aom_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_order_executed_with_price(&mut self) {
        let filename = format!("{}/OrderExecutedWithPrice_{:06}.arrow", self.output_dir, self.file_counters[6]);
        self.file_counters[6] += 1;
        let sl = std::mem::replace(&mut self.oep_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.oep_timestamp, Vec::with_capacity(self.batch_size));
        let pr = std::mem::replace(&mut self.oep_price, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts)), Arc::new(Int32Array::from(pr))], vec!["stock_locate", "timestamp", "price"]);
    }

    fn flush_cross_trade(&mut self) {
        let filename = format!("{}/CrossTrade_{:06}.arrow", self.output_dir, self.file_counters[7]);
        self.file_counters[7] += 1;
        let sl = std::mem::replace(&mut self.ct_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.ct_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_trade(&mut self) {
        let filename = format!("{}/Trade_{:06}.arrow", self.output_dir, self.file_counters[8]);
        self.file_counters[8] += 1;
        let sl = std::mem::replace(&mut self.t_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.t_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_order_delete(&mut self) {
        let filename = format!("{}/OrderDelete_{:06}.arrow", self.output_dir, self.file_counters[9]);
        self.file_counters[9] += 1;
        let sl = std::mem::replace(&mut self.od_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.od_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_order_replace(&mut self) {
        let filename = format!("{}/OrderReplace_{:06}.arrow", self.output_dir, self.file_counters[10]);
        self.file_counters[10] += 1;
        let sl = std::mem::replace(&mut self.or_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.or_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_stock_trading_action(&mut self) {
        let filename = format!("{}/StockTradingAction_{:06}.arrow", self.output_dir, self.file_counters[11]);
        self.file_counters[11] += 1;
        let sl = std::mem::replace(&mut self.sta_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.sta_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_order_priority_update_y(&mut self) {
        let filename = format!("{}/OrderPriorityUpdateY_{:06}.arrow", self.output_dir, self.file_counters[12]);
        self.file_counters[12] += 1;
        let sl = std::mem::replace(&mut self.opu_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.opu_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    fn flush_net_order_imbalance_indicator(&mut self) {
        let filename = format!("{}/NetOrderImbalanceIndicator_{:06}.arrow", self.output_dir, self.file_counters[13]);
        self.file_counters[13] += 1;
        let sl = std::mem::replace(&mut self.noi_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.noi_timestamp, Vec::with_capacity(self.batch_size));
        let im = std::mem::replace(&mut self.noi_imbalance_shares, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts)), Arc::new(Int64Array::from(im))], vec!["stock_locate", "timestamp", "imbalance_shares"]);
    }

    fn flush_market_participant_position(&mut self) {
        let filename = format!("{}/MarketParticipantPosition_{:06}.arrow", self.output_dir, self.file_counters[14]);
        self.file_counters[14] += 1;
        let sl = std::mem::replace(&mut self.mpp_stock_locate, Vec::with_capacity(self.batch_size));
        let ts = std::mem::replace(&mut self.mpp_timestamp, Vec::with_capacity(self.batch_size));
        self.write_arrow(&filename, vec![Arc::new(Int32Array::from(sl)), Arc::new(Int64Array::from(ts))], vec!["stock_locate", "timestamp"]);
    }

    /// ⚡ Zero-Copy Direct Mem blit serialization using Arrow IPC File format
    fn write_arrow(&self, filename: &str, arrays: Vec<Arc<dyn Array>>, column_names: Vec<&str>) {
        let fields: Vec<Field> = column_names
            .iter()
            .enumerate()
            .map(|(i, name)| Field::new(*name, arrays[i].data_type().clone(), true))
            .collect();

        let schema = Arc::new(Schema::new(fields));
        let batch = RecordBatch::try_new(schema.clone(), arrays).expect("Failed to create record batch");

        let file = File::create(filename).expect("Failed to create file");

        // Use Arrow IPC FileWriter instead of Parquet's ArrowWriter
        let mut writer = FileWriter::try_new(file, &schema).expect("Failed to create Arrow IPC writer");
        writer.write(&batch).expect("Failed to write Arrow record batch");
        writer.finish().expect("Failed to finish Arrow file serialization");
    }

    pub fn flush_remaining(&mut self) {
        if !self.se_timestamp.is_empty() { self.flush_system_event(); }
        if !self.sd_stock_locate.is_empty() { self.flush_stock_directory(); }
        if !self.ao_stock_locate.is_empty() { self.flush_add_order(); }
        if !self.oe_stock_locate.is_empty() { self.flush_order_executed(); }
        if !self.oc_stock_locate.is_empty() { self.flush_order_cancel(); }
        if !self.aom_stock_locate.is_empty() { self.flush_add_order_mpid(); }
        if !self.oep_stock_locate.is_empty() { self.flush_order_executed_with_price(); }
        if !self.ct_stock_locate.is_empty() { self.flush_cross_trade(); }
        if !self.t_stock_locate.is_empty() { self.flush_trade(); }
        if !self.od_stock_locate.is_empty() { self.flush_order_delete(); }
        if !self.or_stock_locate.is_empty() { self.flush_order_replace(); }
        if !self.sta_stock_locate.is_empty() { self.flush_stock_trading_action(); }
        if !self.opu_stock_locate.is_empty() { self.flush_order_priority_update_y(); }
        if !self.noi_stock_locate.is_empty() { self.flush_net_order_imbalance_indicator(); }
        if !self.mpp_stock_locate.is_empty() { self.flush_market_participant_position(); }
    }
}