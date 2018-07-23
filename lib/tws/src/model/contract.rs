pub struct DeltaNeutralContract {
    pub con_id: i32,
    pub delta: f64,
    pub price: f64,
}

impl DeltaNeutralContract {
    pub fn new() -> Self {
        DeltaNeutralContract {
            con_id: -1,
            delta: 0.0,
            price: 0.0,
        }
    }
}
pub struct ComboLeg {
    pub con_id: i32,
    pub ratio: i32,
    pub action: String,
    pub exchange: String,
    pub open_close: i32,
    pub short_sale_slot: i32,
    pub designated_location: String,
    pub exempt_code: i32,
}
pub struct Contract {
    pub con_id: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: f64,
    pub right: String,
    pub multiplier: String,
    pub exchange: String,
    pub primary_exch: String,
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
    pub sec_id_type: String,
    pub sec_id: String,

    pub under_comp: Option<DeltaNeutralContract>,
    pub include_expired: bool,
    pub combo_legs_descrip: String,
    pub combo_legs: Vec<ComboLeg>,
}

impl Contract {
    /*fn new() -> Self {
        Contract {
            con_id: 0,
            strike: 0,
            include_expired: false,
        }
    }*/
}
