use model::contract::*;
use model::tagvalue::*;

pub struct ContractDetails {
    pub contract: Contract,
    pub market_name: String,
    pub min_tick: f64,
    pub price_magnifier: i32,
    pub order_types: String,
    pub valid_exchanges: String,
    pub under_con_id: i32,
    pub long_name: String,
    pub contract_month: String,
    pub industry: String,
    pub category: String,
    pub sub_category: String,
    pub time_zone_id: String,
    pub trading_hours: String,
    pub liquid_hours: String,
    pub ev_rule: String,
    pub ev_multiplier: f64,
    pub sec_id_list: Vec<TagValue>,

    // BOND values
    pub cusip: String,
    pub ratings: String,
    pub desc_append: String,
    pub bond_type: String,
    pub coupon_type: String,
    pub callable: bool,
    pub putable: bool,
    pub coupon: f64,
    pub convertible: bool,
    pub maturity: String,
    pub issue_date: String,
    pub next_option_date: String,
    pub next_option_type: String,
    pub next_option_partial: bool,
    pub notes: String,
}

/*impl Default for ContractDetails {
    fn default() -> Self {}
}*/

impl ContractDetails {
    //fn new() -> Self {}
}
