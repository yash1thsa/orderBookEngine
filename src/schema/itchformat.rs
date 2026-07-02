#[derive(Debug)]
pub enum ItchMessage {
    SystemEvent(SystemEventMessage),
    StockDirectory(StockDirectoryMessage),
    StockTradingAction(StockTradingActionMessage),

    AddOrder(AddOrderMessage),
    AddOrderMPID(AddOrderMPIDMessage),

    OrderExecuted(OrderExecutedMessage),
    OrderExecutedWithPrice(OrderExecutedWithPriceMessage),

    OrderCancel(OrderCancelMessage),
    OrderDelete(OrderDeleteMessage),
    OrderReplace(OrderReplaceMessage),

    Trade(TradeMessage),
    CrossTrade(CrossTradeMessage),
    OrderPriorityUpdateY(OrderPriorityUpdateYMessage),
    NetOrderImbalanceIndicator(NetOrderImbalanceIndicatorMessage),
    MarketParticipantPosition(MarketParticipantPositionMessage),

    Unknown(UnknownMessage),
}

impl ItchMessage {
    pub fn name(&self) -> &'static str {
        match self {
            ItchMessage::SystemEvent(_) => "SystemEvent",
            ItchMessage::StockDirectory(_) => "StockDirectory",
            ItchMessage::StockTradingAction(_) => "StockTradingAction",

            ItchMessage::AddOrder(_) => "AddOrder",
            ItchMessage::AddOrderMPID(_) => "AddOrderMPID",

            ItchMessage::OrderExecuted(_) => "OrderExecuted",
            ItchMessage::OrderExecutedWithPrice(_) => "OrderExecutedWithPrice",

            ItchMessage::OrderCancel(_) => "OrderCancel",
            ItchMessage::OrderDelete(_) => "OrderDelete",
            ItchMessage::OrderReplace(_) => "OrderReplace",

            ItchMessage::Trade(_) => "Trade",
            ItchMessage::CrossTrade(_) => "CrossTrade",
            ItchMessage::OrderPriorityUpdateY(_) => "OrderPriorityUpdateY",
            ItchMessage::NetOrderImbalanceIndicator(_) => "NetOrderImbalanceIndicator",
            ItchMessage::MarketParticipantPosition(_) => "MarketParticipantPosition",

            ItchMessage::Unknown(_) => "Unknown",
        }
    }

    pub fn timestamp(&self) -> Option<u64> {
        match self {
            ItchMessage::SystemEvent(m) => Some(m.timestamp),
            ItchMessage::StockDirectory(m) => Some(m.timestamp),
            ItchMessage::StockTradingAction(m) => Some(m.timestamp),
            ItchMessage::AddOrder(m) => Some(m.timestamp),
            ItchMessage::AddOrderMPID(m) => Some(m.timestamp),
            ItchMessage::OrderExecuted(m) => Some(m.timestamp),
            ItchMessage::OrderExecutedWithPrice(m) => Some(m.timestamp),
            ItchMessage::OrderCancel(m) => Some(m.timestamp),
            ItchMessage::OrderDelete(m) => Some(m.timestamp),
            ItchMessage::OrderReplace(m) => Some(m.timestamp),
            ItchMessage::Trade(m) => Some(m.timestamp),
            ItchMessage::CrossTrade(m) => Some(m.timestamp),
            ItchMessage::OrderPriorityUpdateY(m) => Some(m.timestamp),
            ItchMessage::NetOrderImbalanceIndicator(m) => Some(m.timestamp),
            ItchMessage::MarketParticipantPosition(m) => Some(m.timestamp),
            ItchMessage::Unknown(_) => None,
        }
    }
}

/* =========================
   CORE MESSAGES
   ========================= */

#[derive(Debug)]
pub struct SystemEventMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub event_code: u8,
}

#[derive(Debug)]
pub struct OrderPriorityUpdateYMessage {
    pub stock_locate: u16,
    pub tracking_number: u32,
    pub timestamp: u64,
    pub payload: [u8; 7],
}

#[derive(Debug)]
pub struct NetOrderImbalanceIndicatorMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,

    pub paired_shares: u64,
    pub imbalance_shares: u64,
    pub imbalance_direction: u8,

    pub stock: [u8; 8],

    pub far_price: u32,
    pub near_price: u32,
    pub current_reference_price: u32,

    pub cross_type: u8,
    pub price_variation_indicator: u8,
}

#[derive(Debug)]
pub struct MarketParticipantPositionMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,

    pub stock: [u8; 8],

    pub primary_market_maker: u8,
    pub market_maker_mode: u8,
    pub market_participant_state: u8,
}

#[derive(Debug)]
pub struct StockDirectoryMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub symbol: [u8; 8],
    pub market_category: u8,
    pub financial_status: u8,
    pub round_lot_size: u32,
    pub round_lots_only: u8,
}

#[derive(Debug)]
pub struct StockTradingActionMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub stock: [u8; 8],
    pub trading_state: u8,
    pub reserved: u8,
    pub reason: u8,
}

/* =========================
   ORDER ENTRY
   ========================= */

#[derive(Debug)]
pub struct AddOrderMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub buy_sell_indicator: u8,
    pub shares: u32,
    pub stock: [u8; 8],
    pub price: u32,
}

#[derive(Debug)]
pub struct AddOrderMPIDMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub buy_sell_indicator: u8,
    pub shares: u32,
    pub stock: [u8; 8],
    pub price: u32,
    pub attribution: [u8; 4],
}

/* =========================
   ORDER EVENTS
   ========================= */

#[derive(Debug)]
pub struct OrderExecutedMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub executed_shares: u32,
    pub match_number: u64,
}

#[derive(Debug)]
pub struct OrderExecutedWithPriceMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub executed_shares: u32,
    pub match_number: u64,
    pub printable: u8,
    pub execution_price: u32,
}

#[derive(Debug)]
pub struct OrderCancelMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub cancelled_shares: u32,
}

#[derive(Debug)]
pub struct OrderDeleteMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
}

#[derive(Debug)]
pub struct OrderReplaceMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub original_order_reference_number: u64,
    pub new_order_reference_number: u64,
    pub shares: u32,
    pub price: u32,
}

/* =========================
   TRADE MESSAGES
   ========================= */

#[derive(Debug)]
pub struct TradeMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub buy_sell_indicator: u8,
    pub shares: u32,
    pub stock: [u8; 8],
    pub price: u32,
    pub match_number: u64,
}

#[derive(Debug)]
pub struct CrossTradeMessage {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub shares: u64,
    pub stock: [u8; 8],
    pub cross_price: u32,
    pub match_number: u64,
    pub cross_type: u8,
}

/* =========================
   UNKNOWN
   ========================= */

#[derive(Debug)]
pub struct UnknownMessage {
    pub message_type: u8,
    pub body: Vec<u8>,
}