use model::order_condition::*;
use model::tagvalue::*;

pub enum OrderStatus {
    ApiPending,
    ApiCancelled,
    PreSubmitted,
    PendingCancel,
    Cancelled,
    Submitted,
    Filled,
    Inactive,
    PendingSubmit,
    Unknown,
}
pub enum OrderType {
    None,
    MKT,
    LMT,
    STP,
    STP_LMT,
    REL,
    TRAIL,
    BOX_TOP,
    FIX_PEGGED,
    LIT,
    LMT_PLUS_MKT,
    LOC,
    MIT,
    MKT_PRT,
    MOC,
    MTL,
    PASSV_REL,
    PEG_BENCH,
    PEG_MID,
    PEG_MKT,
    PEG_PRIM,
    PEG_STK,
    REL_PLUS_LMT,
    REL_PLUS_MKT,
    SNAP_MID,
    SNAP_MKT,
    SNAP_PRIM,
    STP_PRT,
    TRAIL_LIMIT,
    TRAIL_LIT,
    TRAIL_LMT_PLUS_MKT,
    TRAIL_MIT,
    TRAIL_REL_PLUS_MKT,
    VOL,
    VWAP,
    QUOTE,
    PEG_PRIM_VOL,
    PEG_MID_VOL,
    PEG_MKT_VOL,
    PEG_SRF_VOL,
}

pub struct OrderComboLeg {
    pub price: f64,
}

pub struct SoftDollarTier {
    pub name: String,
    pub value: String,
    pub display_name: String,
}

pub struct Order {
    // order id's
    pub client_id: i32,
    pub order_id: i32,
    pub perm_id: i64,
    pub parent_id: i32,

    // primary attributes
    pub action: String,
    pub total_quantity: f64,
    pub display_size: i32,
    pub order_type: String,
    pub lmt_price: f64,
    pub aux_price: f64,
    pub tif: String,

    // Clearing info
    pub account: String,
    pub settling_firm: String,
    pub clearing_account: String,
    pub clearing_intent: String,

    // Secondary attributes
    pub all_or_none: bool,
    pub block_order: bool,
    pub hidden: bool,
    pub outside_rth: bool,
    pub sweep_to_fill: bool,
    pub percent_offset: f64,
    pub trailing_percent: f64,
    pub trail_stop_price: f64,
    pub min_qty: i32,
    pub good_after_time: String,
    pub good_till_date: String,
    pub oca_group: String,
    pub order_ref: String,
    pub rule_80a: String,
    pub oca_type: i32,
    pub trigger_method: i32,

    // extended order fiedls
    pub active_start_time: String,
    pub active_stop_time: String,

    // advisor allocation orders
    pub fa_group: String,
    pub fa_method: String,
    pub fa_percentage: String,
    pub fa_profile: String,

    // volatility orders
    pub volatility: f64,
    pub voltility_type: i32,
    pub continuous_update: i32,
    pub reference_price_type: i32,
    pub delta_neutral_order_type: String,
    pub delta_neutral_aux_price: f64,
    pub delta_neutral_con_id: i32,
    pub delta_neutral_open_close: String,
    pub delta_neutral_short_sale: bool,

    // scale orders
    pub scale_init_level_size: i32,
    pub scale_subs_level_size: i32,
    pub scale_price_increment: f64,
    pub scale_price_adjust_value: f64,
    pub scale_price_adjust_interval: i32,
    pub scale_profit_offset: f64,
    pub scale_auto_reset: bool,
    pub scale_init_position: i32,
    pub scale_init_fill_qty: i32,
    pub scale_random_percent: bool,
    pub scale_table: String,

    // hedge orders
    pub hedge_type: String,
    pub hedge_param: String,

    // algo orders
    pub algo_strategy: String,
    pub algo_params: Vec<TagValue>,
    pub algo_id: String,

    // combo orders
    pub smart_combo_routing_params: Vec<TagValue>,
    pub order_combo_legs: Vec<OrderComboLeg>,

    // processing control
    pub what_if: bool,
    pub transmit: bool,
    pub override_percentage_constraints: bool,

    // Institutional orders only
    pub open_close: String,
    pub origin: i32,
    pub short_sale_slot: i32,
    pub designated_location: String,
    pub exempt_code: i32,
    pub delta_neutral_settling_firm: String,
    pub delta_neutral_clearing_account: String,
    pub delta_neutral_clearing_intent: String,

    // SMART routing only
    pub discretionary_amt: f64,
    pub etrade_only: bool,
    pub firm_quote_only: bool,
    pub nbbo_price_cap: f64,
    pub opt_out_smart_routing: bool,

    // BOX or VOL ORDERS ONLY
    pub auction_strategy: i32,

    // BOX ORDER ONLY
    pub staring_price: f64,
    pub stock_ref_price: f64,
    pub delta: f64,

    // pegged to stock or VOL orders
    pub stock_range_lower: f64,
    pub stock_range_upper: f64,

    // COMBO ORDERS ONLY
    pub basis_points: f64,      // EFP order only
    pub basis_points_type: i32, // EFP order only

    // Not Held
    pub not_held: bool,
    pub order_misc_options: Vec<TagValue>,

    // order algo id
    pub solicited: bool,
    pub randomize_size: bool,
    pub randomize_price: bool,

    // VER PEG2BENCH fields
    pub reference_contract_id: i32,
    pub pegged_change_amount: f64,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount: f64,
    pub reference_exchange_id: String,
    pub adjusted_order_type: OrderType,
    pub trigger_price: f64,
    pub adjusted_stop_price: f64,
    pub adjusted_stop_limit_price: f64,
    pub adjusted_trailing_amount: f64,
    pub adjustable_trailing_unit: i32,
    pub lmt_price_offset: f64,

    pub conditions: Vec<OrderCondition>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth: bool,

    // models
    pub model_code: String,
    pub soft_sollar_tier: SoftDollarTier,
}
