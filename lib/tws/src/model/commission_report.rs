use penny::Currency;

pub struct CommissionReport {
    pub exec_id: String,
    pub commission: f64,
    pub currency: Currency,
    pub realized_pnl: f64,
    pub yield_value: f64,
    pub yield_redemption_date: i32,
}

pub struct MarketDataType {
    pub req_id: i32,
    pub market_data_type: i32,
}

pub struct ErrorMsg {
    pub id: i32,
    pub code: i32,
    pub message: String,
}

pub struct AccountValue {
    pub account: Option<String>,
    pub key: String,
    pub value: String,
    pub cur: String,
}
