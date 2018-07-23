use decoder::wire::*;
use contract_detail::*;
use decoder::tagvalue::*;
use contract::*;
use decoder::tagvalue::*;
use tagvalue::*;

pub struct ContractMsg {
    req_id: i32,
    contract_detail: ContractDetails,
}

named!(decode_contract_data<&[u8], ContractMsg>,
    do_parse!(
        version: int_value >>
        req_id: cond!(version>=3, int_value) >>
        symbol: string_value >>
        sec_type: string_value >>
        last_trade_date_or_contract_month: string_value >>
        strike: double_value >>
        right: string_value >>
        exchange: string_value >>
        currency: string_value >>
        local_symbol: string_value >>
        market_name: string_value >>
        trading_class: string_value >>
        con_id: int_value >>
        min_tick: double_value >>
        multiplier: string_value >>
        order_types: string_value >>
        valid_exchanges: string_value >>
        price_magnifier: cond!(version >= 2, int_value) >>
        under_con_id: cond!(version >= 4, int_value) >>
        long_name: cond!(version >= 5, string_value) >>
        primary_exch: cond!(version >= 5, string_value) >>

        contract_month: cond!(version >= 6, string_value) >>
        industry: cond!(version >= 6, string_value) >>
        category: cond!(version >=6, string_value) >>
        sub_category: cond!(version >= 6, string_value) >>
        time_zone_id: cond!(version>=6, string_value) >>
        trading_hours: cond!(version >=6, string_value) >>
        liquid_hours: cond!(version >=6, string_value) >>

        ev_rule: cond!(version >= 8, string_value) >>
        ev_multiplier: cond!(version >= 8, double_value) >>
        sec_id_list: cond!(version >= 7, decode_tag_value_list) >>
        (
            ContractMsg {
                req_id: req_id.unwrap_or(-1),
                contract_detail: ContractDetails{
                    contract: Contract {
                        con_id: con_id,
                        symbol: symbol.to_string(),
                        sec_type: sec_type.to_string(),
                        last_trade_date_or_contract_month: last_trade_date_or_contract_month.to_string(),
                        strike: strike,
                        right: right.to_string(),
                        multiplier: multiplier.to_string(),
                        exchange: exchange.to_string(),
                        primary_exch: primary_exch.unwrap_or("").to_string(),
                        currency: currency.to_string(),
                        local_symbol: local_symbol.to_string(),
                        trading_class: trading_class.to_string(),
                        sec_id_type: "".to_string(),
                        sec_id: "".to_string(),
                        under_comp: Some(DeltaNeutralContract::new()),
                        include_expired: false,
                        combo_legs_descrip: "".to_string(),
                        combo_legs: Vec::new()
                    },
                    market_name: market_name.to_string(),
                    min_tick: min_tick,
                    price_magnifier: price_magnifier.unwrap_or(-1),
                    order_types: order_types.to_string(),
                    valid_exchanges: "".to_string(),
                    under_con_id: under_con_id.unwrap_or(-1),
                    long_name: long_name.unwrap_or("").to_string(),
                    contract_month: contract_month.unwrap_or("").to_string(),
                    industry: industry.unwrap_or("").to_string(),
                    category: category.unwrap_or("").to_string(),
                    sub_category: sub_category.unwrap_or("").to_string(),
                    time_zone_id: time_zone_id.unwrap_or("").to_string(),
                    trading_hours: trading_hours.unwrap_or("").to_string(),
                    liquid_hours: liquid_hours.unwrap_or("").to_string(),
                    ev_rule: ev_rule.unwrap_or("").to_string(),
                    ev_multiplier: ev_multiplier.unwrap_or(1.0),
                    sec_id_list: sec_id_list.unwrap_or(Vec::new()),
                    // default
                    cusip: "".to_string(),
                    ratings: "".to_string(),
                    desc_append: "".to_string(),
                    bond_type: "".to_string(),
                    coupon_type: "".to_string(),
                    callable: false,
                    putable: false,
                    coupon: 0.0,
                    convertible: false,
                    maturity: "".to_string(),
                    issue_date: "".to_string(),
                    next_option_date: "".to_string(),
                    next_option_type: "".to_string(),
                    next_option_partial: false,
                    notes: "".to_string(),
                }
            }
        )                   

    )
);


named!(decode_bond_contract_data_msg<&[u8],()>,
    do_parse!(
        version: int_value >>
        req_id: cond!(version >= 3, int_value) >>
        symbol: string_value >>
        sec_type: string_value >>
        cusip: string_value >>
        coupon: double_value >>
        maturity: string_value >>
        issue_date: string_value >>
        ratings: string_value >>
        bond_type: string_value >>
        coupon_type: string_value >>
        convertible: bool_value >>
        callable: bool_value >>
        putable: bool_value >>
        desc_append: string_value >>
        exchange: string_value >>
        currency: string_value >>
        market_name: string_value >>
        trading_class: string_value >>
        con_id: int_value >>
        min_tick: double_value >>
        order_type: string_value >>
        valid_exchanges: string_value >>

        next_option_date: cond!(version >= 2, string_value) >>
        next_option_type: cond!(version >= 2, string_value) >>
        next_option_partial: cond!(version >= 2, bool_value) >>
        notes: cond!(version >= 2, string_value) >>


        long_name: cond!(version >= 4, string_value) >>
        ev_rule: cond!(version >= 6, string_value) >>
        ev_multiplier: cond!(version >= 6, double_value) >>

        sec_id_list: cond!(version >= 5, decode_tag_value_list) >>
        ()
    )
);

#[test]
fn test_contract() {

}
