pub struct Execution {
    pub order_id: i32,
    pub client_id: i32,
    pub exec_id: String,
    pub time: String,
    pub acct_number: String,
    pub exchange: String,
    pub side: String,
    pub shares: f64,
    pub price: f64,
    pub perm_id: i32,
    pub liquidation: i32,
    pub cum_qty: i32,
    pub avg_price: f64,
    pub order_ref: String,
    pub ev_rule: String,
    pub ev_multiplier: f64,
    pub model_code: String,
}

pub struct ExecutionFilter {
    pub client_id: i32,
    pub acct_code: String,
    pub time: String,
    pub symbol: String,
    pub sec_type: String,
    pub exchange: String,
    pub side: String,
}
