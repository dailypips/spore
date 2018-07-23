use decoder::constants::*;
use encoder::buf::EncodeBuf;
use model::contract::*;
use model::execution::*;
use model::order::*;
use model::scanner_subscription::*;
use model::tagvalue::*;
use std::{f64, i32};

pub(crate) const REQ_MKT_DATA: i32 = 1;
pub(crate) const CANCEL_MKT_DATA: i32 = 2;
pub(crate) const PLACE_ORDER: i32 = 3;
pub(crate) const CANCEL_ORDER: i32 = 4;
pub(crate) const REQ_OPEN_ORDERS: i32 = 5;
pub(crate) const REQ_ACCOUNT_DATA: i32 = 6;
pub(crate) const REQ_EXECUTIONS: i32 = 7;
pub(crate) const REQ_IDS: i32 = 8;
pub(crate) const REQ_CONTRACT_DATA: i32 = 9;
pub(crate) const REQ_MKT_DEPTH: i32 = 10;
pub(crate) const CANCEL_MKT_DEPTH: i32 = 11;
pub(crate) const REQ_NEWS_BULLETINS: i32 = 12;
pub(crate) const CANCEL_NEWS_BULLETINS: i32 = 13;
pub(crate) const SET_SERVER_LOGLEVEL: i32 = 14;
pub(crate) const REQ_AUTO_OPEN_ORDERS: i32 = 15;
pub(crate) const REQ_ALL_OPEN_ORDERS: i32 = 16;
pub(crate) const REQ_MANAGED_ACCTS: i32 = 17;
pub(crate) const REQ_FA: i32 = 18;
pub(crate) const REPLACE_FA: i32 = 19;
pub(crate) const REQ_HISTORICAL_DATA: i32 = 20;
pub(crate) const EXERCISE_OPTIONS: i32 = 21;
pub(crate) const REQ_SCANNER_SUBSCRIPTION: i32 = 22;
pub(crate) const CANCEL_SCANNER_SUBSCRIPTION: i32 = 23;
pub(crate) const REQ_SCANNER_PARAMETERS: i32 = 24;
pub(crate) const CANCEL_HISTORICAL_DATA: i32 = 25;
pub(crate) const REQ_CURRENT_TIME: i32 = 49;
pub(crate) const REQ_REAL_TIME_BARS: i32 = 50;
pub(crate) const CANCEL_REAL_TIME_BARS: i32 = 51;
pub(crate) const REQ_FUNDAMENTAL_DATA: i32 = 52;
pub(crate) const CANCEL_FUNDAMENTAL_DATA: i32 = 53;
pub(crate) const REQ_CALC_IMPLIED_VOLAT: i32 = 54;
pub(crate) const REQ_CALC_OPTION_PRICE: i32 = 55;
pub(crate) const CANCEL_CALC_IMPLIED_VOLAT: i32 = 56;
pub(crate) const CANCEL_CALC_OPTION_PRICE: i32 = 57;
pub(crate) const REQ_GLOBAL_CANCEL: i32 = 58;
pub(crate) const REQ_MARKET_DATA_TYPE: i32 = 59;
pub(crate) const REQ_POSITIONS: i32 = 61;
pub(crate) const REQ_ACCOUNT_SUMMARY: i32 = 62;
pub(crate) const CANCEL_ACCOUNT_SUMMARY: i32 = 63;
pub(crate) const CANCEL_POSITIONS: i32 = 64;
pub(crate) const VERIFY_REQUEST: i32 = 65;
pub(crate) const VERIFY_MESSAGE: i32 = 66;
pub(crate) const QUERY_DISPLAY_GROUPS: i32 = 67;
pub(crate) const SUBSCRIBE_TO_GROUP_EVENTS: i32 = 68;
pub(crate) const UPDATE_DISPLAY_GROUP: i32 = 69;
pub(crate) const UNSUBSCRIBE_FROM_GROUP_EVENTS: i32 = 70;
pub(crate) const START_API: i32 = 71;
pub(crate) const VERIFY_AND_AUTH_REQUEST: i32 = 72;
pub(crate) const VERIFY_AND_AUTH_MESSAGE: i32 = 73;
pub(crate) const REQ_POSITIONS_MULTI: i32 = 74;
pub(crate) const CANCEL_POSITIONS_MULTI: i32 = 75;
pub(crate) const REQ_ACCOUNT_UPDATES_MULTI: i32 = 76;
pub(crate) const CANCEL_ACCOUNT_UPDATES_MULTI: i32 = 77;
pub(crate) const REQ_SEC_DEF_OPT_PARAMS: i32 = 78;
pub(crate) const REQ_SOFT_DOLLAR_TIERS: i32 = 79;

pub fn start_api(server_version: i32, optional_capabilities: String) -> Option<EncodeBuf> {
    let mut buf = EncodeBuf::new();
    const VERSION: i32 = 2;
    buf.put_int(START_API);
    buf.put_int(VERSION);
    if server_version >= MIN_SERVER_VER_OPTIONAL_CAPABILITIES {
        buf.put_string(&optional_capabilities);
    }
    Some(buf)
}

pub fn cancel_scanner_subscription(server_version: i32, req_id: i32) -> Option<EncodeBuf> {
    if server_version < 24 {
        return None;
    }

    const VERSION: i32 = 1;

    let mut buf = EncodeBuf::new();
    buf.put_int(CANCEL_SCANNER_SUBSCRIPTION);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn req_scanner_parameters(server_version: i32) -> Option<EncodeBuf> {
    if server_version < 24 {
        return None;
    }

    const VERSION: i32 = 1;

    let mut buf = EncodeBuf::new();
    buf.put_int(REQ_SCANNER_PARAMETERS);
    buf.put_int(VERSION);

    Some(buf)
}

pub fn req_scannder_subscription(
    server_version: i32,
    req_id: i32,
    subscribe: &ScannerSubscription,
    options: Vec<TagValue>,
) -> Option<EncodeBuf> {
    if server_version < 24 {
        return None;
    }

    const VERSION: i32 = 4;
    let mut buf = EncodeBuf::new();
    buf.put_int(REQ_SCANNER_SUBSCRIPTION);
    buf.put_int(VERSION);
    buf.put_int(req_id);
    buf.put_int_max(subscribe.number_of_rows);
    buf.put_string(&subscribe.instrument);
    buf.put_string(&subscribe.location_code);
    buf.put_string(&subscribe.scan_code);
    buf.put_f64_max(subscribe.above_price);
    buf.put_f64_max(subscribe.below_price);
    buf.put_int_max(subscribe.above_volume);
    buf.put_f64_max(subscribe.market_cap_above);
    buf.put_f64_max(subscribe.market_cap_below);
    buf.put_string(&subscribe.moody_rating_above);
    buf.put_string(&subscribe.moody_rating_below);
    buf.put_string(&subscribe.sp_rating_above);
    buf.put_string(&subscribe.sp_rating_below);
    buf.put_string(&subscribe.maturity_date_above);
    buf.put_string(&subscribe.maturity_date_below);
    buf.put_f64_max(subscribe.coupon_rate_above);
    buf.put_f64_max(subscribe.coupon_rate_below);
    buf.put_string(&subscribe.exclude_convertible);
    if server_version >= 25 {
        buf.put_int_max(subscribe.average_option_volume_above);
        buf.put_string(&subscribe.scanner_setting_pairs);
    }

    if server_version >= 27 {
        buf.put_string(&subscribe.stock_type_filter);
    }

    if server_version >= MIN_SERVER_VER_LINKING {
        buf.put_tagvalue(&options);
    }
    Some(buf)
}

pub fn req_mkt_data(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    generic_tick_list: String,
    snapshot: bool,
    mkt_data_options: Vec<TagValue>,
) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_SNAPSHOT_MKT_DATA && snapshot {
        return None;
    }

    if server_version < MIN_SERVER_VER_UNDER_COMP {
        match contract.under_comp {
            Some(_) => return None,
            _ => (),
        }
    }

    if server_version < MIN_SERVER_VER_REQ_MKT_DATA_CONID {
        if contract.con_id > 0 {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        if !contract.trading_class.is_empty() {
            return None;
        }
    }

    const VERSION: i32 = 11;

    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_MKT_DATA);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    if server_version >= MIN_SERVER_VER_REQ_MKT_DATA_CONID {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);

    if server_version >= 15 {
        buf.put_string(&contract.multiplier);
    }

    buf.put_string(&contract.exchange);

    if server_version >= 14 {
        buf.put_string(&contract.primary_exch);
    }

    buf.put_string(&contract.currency);

    if server_version >= 2 {
        buf.put_string(&contract.local_symbol);
    }

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    if server_version >= 8 && contract.sec_type.to_uppercase() == "BAG" {
        let count = contract.combo_legs.len() as i32;
        buf.put_int(count);
        for elem in &contract.combo_legs {
            buf.put_int(elem.con_id);
            buf.put_int(elem.ratio);
            buf.put_string(&elem.action);
            buf.put_string(&elem.exchange);
        }
    }

    if server_version >= MIN_SERVER_VER_UNDER_COMP {
        if let Some(ref comp) = contract.under_comp {
            buf.put_bool(true);
            buf.put_int(comp.con_id);
            buf.put_f64(comp.delta);
            buf.put_f64(comp.price);
        } else {
            buf.put_bool(false);
        }
    }

    if server_version >= 31 {
        buf.put_string(&generic_tick_list);
    }

    if server_version >= MIN_SERVER_VER_SNAPSHOT_MKT_DATA {
        buf.put_bool(snapshot);
    }

    if server_version >= MIN_SERVER_VER_LINKING {
        buf.put_tagvalue(&mkt_data_options);
    }

    Some(buf)
}

pub fn cancel_historical_data(server_version: i32, req_id: i32) -> Option<EncodeBuf> {
    if server_version < 24 {
        return None;
    }

    const VERSION: i32 = 1;

    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_HISTORICAL_DATA);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn cancel_realtime_bars(server_version: i32, req_id: i32) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_REAL_TIME_BARS {
        return None;
    }

    const VERSION: i32 = 1;

    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_REAL_TIME_BARS);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn req_historical_data(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    end_date_time: String,
    duration_str: String,
    bar_size_setting: String,
    what_to_show: String,
    use_rth: i32,
    format_date: i32,
    chart_options: &Vec<TagValue>,
) -> Option<EncodeBuf> {
    if server_version < 16 {
        return None;
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        if !contract.trading_class.is_empty() {
            return None;
        }
        if contract.con_id > 0 {
            return None;
        }
    }

    const VERSION: i32 = 6;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_HISTORICAL_DATA);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);
    buf.put_string(&contract.multiplier);
    buf.put_string(&contract.exchange);
    buf.put_string(&contract.primary_exch);
    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    if server_version >= 31 {
        buf.put_bool(contract.include_expired);
    }

    if server_version >= 20 {
        buf.put_string(&end_date_time);
        buf.put_string(&bar_size_setting);
    }

    buf.put_string(&duration_str);
    buf.put_int(use_rth); //TODO
    buf.put_string(&what_to_show);

    if server_version > 16 {
        buf.put_int(format_date);
    }

    if contract.sec_type.to_uppercase() == "BAG" {
        let count = contract.combo_legs.len() as i32;
        buf.put_int(count);
        for elem in &contract.combo_legs {
            buf.put_int(elem.con_id);
            buf.put_int(elem.ratio);
            buf.put_string(&elem.action);
            buf.put_string(&elem.exchange);
        }
    }

    if server_version >= MIN_SERVER_VER_LINKING {
        buf.put_tagvalue(chart_options);
    }

    Some(buf)
}

pub fn req_realtime_bars(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    bar_size: i32,
    what_to_show: String,
    use_rth: bool,
    realtime_bars_option: &Vec<TagValue>,
) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_REAL_TIME_BARS {
        return None;
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        if !contract.trading_class.is_empty() || contract.con_id > 0 {
            return None;
        }
    }

    const VERSION: i32 = 3;

    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_REAL_TIME_BARS);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);
    buf.put_string(&contract.multiplier);
    buf.put_string(&contract.exchange);
    buf.put_string(&contract.primary_exch);
    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);
    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    buf.put_int(bar_size);
    buf.put_string(&what_to_show);
    buf.put_bool(use_rth);

    if server_version >= MIN_SERVER_VER_LINKING {
        buf.put_tagvalue(realtime_bars_option)
    }

    Some(buf)
}

pub fn req_contract_details(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
) -> Option<EncodeBuf> {
    if server_version < 4 {
        return None;
    }

    if server_version < MIN_SERVER_VER_SEC_ID_TYPE {
        if !contract.sec_id_type.is_empty() || !contract.sec_id.is_empty() {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        if !contract.trading_class.is_empty() {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_LINKING {
        if !contract.primary_exch.is_empty() {
            return None;
        }
    }

    const VERSION: i32 = 8;

    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_CONTRACT_DATA);
    buf.put_int(VERSION);

    if server_version >= MIN_SERVER_VER_CONTRACT_DATA_CHAIN {
        buf.put_int(req_id);
    }

    if server_version >= MIN_SERVER_VER_CONTRACT_CONID {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);

    if server_version >= 15 {
        buf.put_string(&contract.multiplier);
    }

    if server_version >= MIN_SERVER_VER_PRIMARYEXCH {
        buf.put_string(&contract.exchange);
        buf.put_string(&contract.primary_exch);
    } else if server_version >= MIN_SERVER_VER_LINKING {
        if !contract.primary_exch.is_empty()
            && (contract.exchange == "BEST" || contract.exchange == "SMART")
        {
            let s = format!("{}:{}", contract.exchange, contract.primary_exch);
            buf.put_string(&s);
        } else {
            buf.put_string(&contract.exchange);
        }
    }

    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    if server_version >= 31 {
        buf.put_bool(contract.include_expired);
    }

    if server_version >= MIN_SERVER_VER_SEC_ID_TYPE {
        buf.put_string(&contract.sec_id_type);
        buf.put_string(&contract.sec_id);
    }

    Some(buf)
}

pub fn req_mkt_depth(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    num_rows: i32,
    options: &Vec<TagValue>,
) -> Option<EncodeBuf> {
    if server_version < 6 {
        return None;
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        if !contract.trading_class.is_empty() || contract.con_id > 0 {
            return None;
        }
    }

    const VERSION: i32 = 5;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_MKT_DEPTH);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);

    if server_version >= 15 {
        buf.put_string(&contract.multiplier);
    }

    buf.put_string(&contract.exchange);
    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    if server_version >= 19 {
        buf.put_int(num_rows);
    }

    if server_version >= MIN_SERVER_VER_LINKING {
        buf.put_tagvalue(options);
    }

    Some(buf)
}

pub fn cancel_mkt_data(req_id: i32) -> Option<EncodeBuf> {
    const VERSION: i32 = 1;

    let mut buf = EncodeBuf::new();
    buf.put_int(CANCEL_MKT_DATA);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn cancel_mkt_depth(server_version: i32, req_id: i32) -> Option<EncodeBuf> {
    if server_version < 6 {
        return None;
    }

    const VERSION: i32 = 1;

    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_MKT_DEPTH);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn exercise_options(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    exercise_action: i32,
    exercise_quantity: i32,
    account: String,
    over_ride: i32,
) -> Option<EncodeBuf> {
    if server_version < 21 {
        return None;
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        return None;
    }

    const VERSION: i32 = 2;
    let mut buf = EncodeBuf::new();

    buf.put_int(EXERCISE_OPTIONS);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);

    buf.put_string(&contract.multiplier);

    buf.put_string(&contract.exchange);
    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    buf.put_int(exercise_action);
    buf.put_int(exercise_quantity);
    buf.put_string(&account);
    buf.put_int(over_ride);

    Some(buf)
}

pub fn place_order(
    server_version: i32,
    id: i32,
    contract: &Contract,
    order: &Order,
) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_SCALE_ORDERS {
        if order.scale_init_level_size != i32::MAX || order.scale_price_increment != f64::MAX {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_SSHORT_COMBO_LEGS {
        if !contract.combo_legs.is_empty() {
            //TODO
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_WHAT_IF_ORDERS {
        if order.what_if {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_UNDER_COMP {
        if let Some(_) = contract.under_comp {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_SCALE_ORDERS2 {
        if order.scale_subs_level_size != i32::MAX {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_ALGO_ORDERS {
        if !order.algo_strategy.is_empty() {
            return None;
        }
    }

    if server_version < MIN_SERVER_VER_NOT_HELD {
        if order.not_held {
            return None;
        }
    }

    //TODO

    let mut buf = EncodeBuf::new();

    Some(buf)
}

pub fn req_account_updates(
    server_version: i32,
    subscribe: bool,
    acct_code: String,
) -> Option<EncodeBuf> {
    const VERSION: i32 = 2;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_ACCOUNT_DATA);
    buf.put_int(VERSION);
    buf.put_bool(subscribe);

    if server_version >= 9 {
        buf.put_string(&acct_code);
    }

    Some(buf)
}
pub fn req_executions(
    server_version: i32,
    req_id: i32,
    filter: ExecutionFilter,
) -> Option<EncodeBuf> {
    const VERSION: i32 = 3;

    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_EXECUTIONS);
    buf.put_int(VERSION);

    if server_version >= MIN_SERVER_VER_EXECUTION_DATA_CHAIN {
        buf.put_int(req_id);
    }

    if server_version >= 9 {
        buf.put_int(filter.client_id);
        buf.put_string(&filter.acct_code);
        buf.put_string(&filter.time);
        buf.put_string(&filter.symbol);
        buf.put_string(&filter.sec_type);
        buf.put_string(&filter.exchange);
        buf.put_string(&filter.side);
    }

    Some(buf)
}

pub fn cancel_order(id: i32) -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_ORDER);
    buf.put_int(VERSION);
    buf.put_int(id);

    Some(buf)
}

pub fn req_open_orders() -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_OPEN_ORDERS);
    buf.put_int(VERSION);

    Some(buf)
}

pub fn req_ids(num_ids: i32) -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();
    buf.put_int(REQ_IDS);
    buf.put_int(VERSION);
    buf.put_int(num_ids);

    Some(buf)
}
pub fn req_news_bulletings(all_msgs: bool) -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_NEWS_BULLETINS);
    buf.put_int(VERSION);
    buf.put_bool(all_msgs);

    Some(buf)
}
pub fn cancel_news_bulleting() -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_NEWS_BULLETINS);
    buf.put_int(VERSION);

    Some(buf)
}
pub fn set_server_log_level(log_level: i32) -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(SET_SERVER_LOGLEVEL);
    buf.put_int(VERSION);
    buf.put_int(log_level);

    Some(buf)
}

pub fn req_auto_open_orders(auto_bind: bool) -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_AUTO_OPEN_ORDERS);
    buf.put_int(VERSION);
    buf.put_bool(auto_bind);

    Some(buf)
}

pub fn req_all_open_orders() -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_ALL_OPEN_ORDERS);
    buf.put_int(VERSION);

    Some(buf)
}

pub fn req_managed_accts() -> Option<EncodeBuf> {
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_MANAGED_ACCTS);
    buf.put_int(VERSION);

    Some(buf)
}

pub fn request_fa(server_version: i32, fa_data_type: i32) -> Option<EncodeBuf> {
    if server_version < 13 {
        return None;
    }
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_FA);
    buf.put_int(VERSION);
    buf.put_int(fa_data_type);

    Some(buf)
}
pub fn replace_fa(server_version: i32, fa_data_type: i32, xml: String) -> Option<EncodeBuf> {
    if server_version < 13 {
        return None;
    }
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REPLACE_FA);
    buf.put_int(VERSION);
    buf.put_int(fa_data_type);
    buf.put_string(&xml);

    Some(buf)
}

pub fn req_current_time(server_version: i32) -> Option<EncodeBuf> {
    if server_version < 33 {
        return None;
    }
    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_CURRENT_TIME);
    buf.put_int(VERSION);

    Some(buf)
}

pub fn req_fundamental_data(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    report_type: String,
) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_FUNDAMENTAL_DATA {
        return None;
    }

    if server_version < MIN_SERVER_VER_TRADING_CLASS {
        if contract.con_id > 0 {
            return None;
        }
    }

    const VERSION: i32 = 2;
    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_FUNDAMENTAL_DATA);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_int(contract.con_id);
    }

    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.exchange);
    buf.put_string(&contract.primary_exch);
    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);

    buf.put_string(&report_type);

    Some(buf)
}

pub fn cancel_fundamental_data(server_version: i32, req_id: i32) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_FUNDAMENTAL_DATA {
        return None;
    }

    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_FUNDAMENTAL_DATA);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn calculate_implied_volatility(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    option_price: f64,
    under_price: f64,
) -> Option<EncodeBuf> {
    const VERSION: i32 = 2;

    let mut buf = EncodeBuf::new();

    buf.put_int(REQ_CALC_IMPLIED_VOLAT);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    buf.put_int(contract.con_id);
    buf.put_string(&contract.symbol);
    buf.put_string(&contract.sec_type);
    buf.put_string(&contract.last_trade_date_or_contract_month);
    buf.put_f64(contract.strike);
    buf.put_string(&contract.right);
    buf.put_string(&contract.multiplier);
    buf.put_string(&contract.exchange);
    buf.put_string(&contract.primary_exch);
    buf.put_string(&contract.currency);
    buf.put_string(&contract.local_symbol);

    if server_version >= MIN_SERVER_VER_TRADING_CLASS {
        buf.put_string(&contract.trading_class);
    }

    buf.put_f64(option_price);
    buf.put_f64(under_price);

    Some(buf)
}

pub fn cancel_calculate_implied_volatility(server_version: i32, req_id: i32) -> Option<EncodeBuf> {
    if server_version < MIN_SERVER_VER_CANCEL_CALC_IMPLIED_VOLAT {
        return None;
    }

    const VERSION: i32 = 1;
    let mut buf = EncodeBuf::new();

    buf.put_int(CANCEL_CALC_IMPLIED_VOLAT);
    buf.put_int(VERSION);
    buf.put_int(req_id);

    Some(buf)
}

pub fn calculate_option_price(
    server_version: i32,
    req_id: i32,
    contract: &Contract,
    volatility: f64,
    under_price: f64,
) -> Option<EncodeBuf> {
    None
}

pub fn cancel_calculate_option_price(req_id: i32) -> Option<EncodeBuf> {
    None
}

pub fn req_global_cancel() -> Option<EncodeBuf> {
    None
}

pub fn req_market_data_type(market_data_type: i32) -> Option<EncodeBuf> {
    None
}

pub fn req_positions() -> Option<EncodeBuf> {
    None
}

pub fn req_sec_def_opt_params(
    req_id: i32,
    underlying_symbol: String,
    fut_fop_exchange: String,
    underlyingSecType: String,
    underlyingConId: i32,
) -> Option<EncodeBuf> {
    None
}

pub fn req_soft_dollar_tiers(req_id: i32) -> Option<EncodeBuf> {
    None
}

pub fn cancel_positions() -> Option<EncodeBuf> {
    None
}

pub fn req_positions_multi(req_id: i32, account: String, model_code: String) -> Option<EncodeBuf> {
    None
}

pub fn cancel_positions_multi(req_id: i32) -> Option<EncodeBuf> {
    None
}

pub fn cancel_account_updates_multi(req_id: i32) -> Option<EncodeBuf> {
    None
}

pub fn req_account_updates_multi(
    req_id: i32,
    account: String,
    model_code: String,
    ledgerAndNLV: bool,
) -> Option<EncodeBuf> {
    None
}

pub fn req_account_summary(req_id: i32, group: String, tags: String) -> Option<EncodeBuf> {
    None
}

pub fn cancel_account_summary(req_id: i32) -> Option<EncodeBuf> {
    None
}

pub fn verify_request(api_name: String, api_version: String) -> Option<EncodeBuf> {
    None
}

pub fn verify_message(api_data: String) -> Option<EncodeBuf> {
    None
}

pub fn verify_and_auth_request(
    api_name: String,
    api_version: String,
    opaqueIsvKey: String,
) -> Option<EncodeBuf> {
    None
}

pub fn verify_and_auth_message(api_data: String, xyz_response: String) -> Option<EncodeBuf> {
    None
}

pub fn query_display_groups(req_id: i32) -> Option<EncodeBuf> {
    None
}

pub fn subscribe_to_group_event(req_id: i32, group_id: i32) -> Option<EncodeBuf> {
    None
}
pub fn update_display_group(req_id: i32, contract_info: String) -> Option<EncodeBuf> {
    None
}

pub fn ubsubscribe_from_group_events(req_id: i32) -> Option<EncodeBuf> {
    None
}
