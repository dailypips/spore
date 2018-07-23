use decoder::wire::*;
use contract::*;
use order::*;
use order_state::*;
use tagvalue::*;
use decoder::tagvalue::*;
use decoder::constants::*;
use order_condition::*;

use nom::IResult;
use session::Session;
use std::str::{from_utf8, FromStr};
use std::{f64, i32};

named!(decode_combo_leg<&[u8], ComboLeg>,
    do_parse!(
        con_id: int_value >>
        ratio:  int_value >>
        action: string_value >>
        exchange: string_value >>
        open_close: int_value >>
        short_sale_slot: int_value >>
        designated_location: string_value >>
        exempt_code: int_value >>
        (ComboLeg {
            con_id,
            ratio,
            action : action.to_string(),
            exchange : exchange.to_string(),
            open_close,
            short_sale_slot,
            designated_location: designated_location.to_string(),
            exempt_code
        })
    )
);

named!(decode_order_combo_leg<&[u8], OrderComboLeg>,
    do_parse!(
        price: double_max_value >>
        (OrderComboLeg {
            price
        })
    )
);


named!(decode_combo_leg_list<&[u8], Vec<ComboLeg>>,
    length_count!(int_value, decode_combo_leg)
);

named!(decode_order_combo_leg_list<&[u8], Vec<OrderComboLeg>>,
    length_count!(int_value, decode_order_combo_leg)
);

struct OperatorCondition<'a> {
    conjunction: bool,
    is_more: bool,
    value : &'a str
}

struct ContractCondition<'a> {
    operator: OperatorCondition<'a>,
    con_id: i32,
    exchange: &'a str,
}

named!(decode_conjunction<&[u8], bool>,
    do_parse!(
        conjunction: string_value >>
        ( conjunction.to_lowercase() == "a" )
    )
);

named!(decode_operator_condition<&[u8], OperatorCondition>,
    do_parse!(
        conjunction: decode_conjunction >>
        is_more: bool_value >>
        value: string_value >>
        (OperatorCondition{
            conjunction, is_more, value,
        })
    )
);

named!(decode_contract_condition<&[u8], ContractCondition>,
    do_parse!(
        operator: decode_operator_condition >>
        con_id : int_value >>
        exchange: string_value >>
        (ContractCondition {
            operator, con_id, exchange
        })
    )
);

named!(decode_execution_condition<&[u8], OrderCondition>,
    do_parse!(
        is_conjunction_connection: decode_conjunction >>
        sec_type: string_value >>
        exchange: string_value >>
        symbol: string_value >>
        (OrderCondition::Execution {
            is_conjunction_connection, 
            sec_type: sec_type.to_string(), 
            exchange: exchange.to_string(), 
            symbol: symbol.to_string()
        })
    )
);

named!(decode_margin_condition<&[u8], OrderCondition>,
    do_parse!(
        operator: decode_operator_condition >>
        (OrderCondition::Margin{
            is_conjunction_connection: operator.conjunction,
            is_more: operator.is_more,
            percent: i32::from_str_radix(operator.value, 10).unwrap(),
        })
    )
);


named!(decode_percent_change_condition<&[u8], OrderCondition>,
    do_parse!(
        contract: decode_contract_condition >>
        (OrderCondition::PercentChange {
            is_conjunction_connection: contract.operator.conjunction,
            is_more: contract.operator.is_more,
            con_id: contract.con_id,
            exchange: contract.exchange.to_string(),
            change_percent: f64::from_str(contract.operator.value).unwrap(),
        })
    )
);


named!(decode_price_condition<&[u8], OrderCondition>,
    do_parse!(
        contract: decode_contract_condition >>
        trigger_mode: int_value >>
        (OrderCondition::Price {
            is_conjunction_connection: contract.operator.conjunction,
            is_more: contract.operator.is_more,
            con_id: contract.con_id,
            exchange: contract.exchange.to_string(),
            price: f64::from_str(contract.operator.value).unwrap(),
            trigger_mode,
        })
    )
);


named!(decode_time_condition<&[u8], OrderCondition>,
    do_parse!(
        operator: decode_operator_condition >>
        (OrderCondition::Time {
            is_conjunction_connection: operator.conjunction,
            is_more: operator.is_more,
            time: operator.value.to_string(),
        })
    )
);

named!(decode_volume_condition<&[u8], OrderCondition>,
    do_parse!(
        contract: decode_contract_condition >>
        (OrderCondition::Volume {
            is_conjunction_connection: contract.operator.conjunction,
            is_more: contract.operator.is_more,
            con_id: contract.con_id,
            exchange: contract.exchange.to_string(),
            volume: i32::from_str_radix(contract.operator.value, 10).unwrap(),
        })
    )
);

const PriceCondtion : i32 = 1;
const TimeCondtion: i32   = 3;
const MarginCondition: i32 = 4;
const ExecutionCondition: i32 = 5;
const VolumeCondition: i32 = 6;
const PercentChangeCondition: i32 = 7;

named!(decode_condition<&[u8], OrderCondition>,
    switch!(int_value,
            PriceCondtion => call!(decode_price_condition) |
            TimeCondtion  => call!(decode_time_condition)  | 
            MarginCondition => call!(decode_margin_condition) |
            ExecutionCondition => call!(decode_execution_condition) |
            VolumeCondition => call!(decode_volume_condition) |
            PercentChangeCondition => call!(decode_percent_change_condition) 
            //_ => IResult::Error(0)
   )
 );

struct Conditions {
    conditions: Vec<OrderCondition>,
    conditions_ignore_rth: bool,
    conditions_cancel_order: bool,
}

named!(decode_conditions<&[u8], Conditions>,
   do_parse!(
        conditions: length_count!(int_value, decode_condition) >>
        conditions_ignore_rth: bool_value >>
        conditions_cancel_order: bool_value >>
        (
            Conditions {
                conditions: conditions,
                conditions_ignore_rth: conditions_ignore_rth,
                conditions_cancel_order: conditions_cancel_order
            }
        )
    )
);

struct OrderStateRef<'a> {
    status: &'a str,
    init_margin: &'a str,
    maint_margin: &'a str,
    equity_with_loan: &'a str,
    commission:f64,
    min_commission:f64,
    max_commission:f64,
    commission_currency: &'a str,
    warning_text: &'a str
}

named!(decode_order_state<&[u8], OrderStateRef>,
    do_parse!(
        status: string_value >>
        init_margin: string_value >>
        maint_margin: string_value >>
        equity_with_loan: string_value >>
        commission: double_max_value >>
        min_commission: double_max_value >>
        max_commission: double_max_value >>
        commission_currency: string_value >>
        warning_text: string_value >>
        (
            OrderStateRef {
                status,
                init_margin ,
                maint_margin,
                equity_with_loan,
                commission,
                min_commission,
                max_commission,
                commission_currency,
                warning_text 
            }
        )
    )
);

named!(decode_order_state1<&[u8], OrderState>,
    do_parse!(
        status: string_value >>
        init_margin: string_value >>
        maint_margin: string_value >>
        equity_with_loan: string_value >>
        commission: double_max_value >>
        min_commission: double_max_value >>
        max_commission: double_max_value >>
        commission_currency: string_value >>
        warning_text: string_value >>
        (
            OrderState {
                status : status.to_string(),
                init_margin : init_margin.to_string(),
                maint_margin : maint_margin.to_string(),
                equity_with_loan: equity_with_loan.to_string(),
                commission,
                min_commission,
                max_commission,
                commission_currency: commission_currency.to_string(),
                warning_text : warning_text.to_string()
            }
        )
    )
);

named!(decode_under_comp<&[u8], Option<DeltaNeutralContract>>,
    do_parse!(
        count: int_value >>
        con_id: cond!(count > 0, int_value) >>
        delta:  cond!(count > 0, double_value) >>
        price:  cond!(count > 0, double_value) >>
        (
            if count > 0 {
                Some(DeltaNeutralContract{
                    con_id: con_id.unwrap(),
                    delta:  delta.unwrap(),
                    price:  price.unwrap()
                })
            }else {
                None
            }
        )
    )
);

struct PegInfo<'a> {
    reference_ctontract_id: i32,
    is_pegged_change_amount_decrease: bool,
    pegged_change_amount: f64,
    reference_change_amout: f64,
    reference_exchange_id: &'a str
}

named!(decode_peg_bench_order<&[u8], PegInfo>,
    do_parse!(
        reference_ctontract_id: int_value >>
        is_pegged_change_amount_decrease: bool_value >>
        pegged_change_amount: double_value >>
        reference_change_amout: double_value >>
        reference_exchange_id: string_value >>
        (PegInfo{
            reference_ctontract_id,
            is_pegged_change_amount_decrease,
            pegged_change_amount,
            reference_change_amout,
            reference_exchange_id
        })
    )
);

named_args!(decode_open_order_msg<'a>(server_version:i32)<&'a [u8], ()>,
    do_parse!(
        version: int_value >>
        order_id: int_value >>
        con_id: cond!(version >=17, int_value) >>
        symbol: string_value >>
        sec_type: string_value >>
        last_trade_date_or_contract_month: string_value >>
        strike: double_value >>
        right: string_value >>
        multiplier: cond!(version >=32, string_value) >>
        exchange: string_value >>
        currency: string_value >>
        local_symbol: cond!(version >=2, string_value) >>
        trading_class: cond!(version >= 32, string_value) >>
        action: string_value >>
        total_quantity: string_value >> 
        order_type: string_value >>
        lmt_price: string_value >> 
        aux_price: string_value >> 
        tif: string_value >>
        oca_group: string_value >>
        account: string_value >>
        open_close: string_value >>
        origin: int_value >>
        order_ref: string_value >>
        client_id: cond!(version >= 3, int_value) >>
        perm_id: cond!(version >=4, int_value) >>
        outside_rth: cond!(version >=4, int_value) >>
        hidden: cond!(version >=4, int_value) >>
        discretionary_amt: cond!(version >=4, double_value) >>
        good_after_time: cond!(version >=5, string_value) >>
        deprecated: cond!(version>=6, string_value) >>
        fa_group: cond!(version >=7, string_value) >>
        fa_method: cond!(version >=7, string_value) >>
        fa_percentage: cond!(version >=7, string_value) >>
        fa_profile: cond!(version >=7, string_value) >>
        model_code: cond!(server_version >=MIN_SERVER_VER_MODELS_SUPPORT, string_value) >>
        good_till_date: cond!(version >= 8, string_value) >>
        rule_80a: cond!(version >= 9, string_value) >>
        percent_offset: cond!(version >= 9, double_max_value) >>
        settling_firm: cond!(version >= 9, string_value) >>
        short_sale_slot: cond!(version >= 9, int_value) >>
        designated_location: cond!(version>=9, string_value) >>
        exempt_code: cond!(version>=23, int_value) >>
        auction_strategy: cond!(version>=9, int_value) >>
        stariting_price: cond!(version>=9, double_max_value) >>
        stock_ref_price: cond!(version>=9, double_max_value) >>
        delta: cond!(version>=9, double_max_value) >>
        stock_range_lower: cond!(version>=9, double_max_value) >>
        stock_range_upper: cond!(version>=9, double_max_value) >>
        display_size: cond!(version>=9, int_value) >>
        rth_only: cond!(version>=9 && version < 18, bool_value) >> // TODO
        block_order: cond!(version>=9, bool_value) >>
        sweep_to_fill: cond!(version>=9, bool_value) >>
        all_or_none: cond!(version>=9, bool_value) >>
        min_qty: cond!(version>=9, int_max_value) >>
        oca_type: cond!(version>=9, int_value) >>
        etrade_only: cond!(version>=9, int_value) >>
        firm_quote_only: cond!(version>=9, bool_value) >>
        nbbo_price_cap: cond!(version>=9, double_max_value) >>

        parent_id: cond!(version>=10, int_value) >>
        trigger_method: cond!(version>=10, int_value) >>
        // version >= 11
        volatility: cond!(version>=11, double_max_value) >>
        volatilly_type: cond!(version>= 11, int_value) >>
        receiverd_int: cond!(version == 11, int_value) >>
        deltaNeutra_order_type: cond!(version>=12, string_value) >>
        deltaNeutra_aux_price: cond!(version>=12, double_max_value) >>
        delta_neutral_con_id: cond!(version >=27 && deltaNeutra_order_type.unwrap_or("").is_empty(),int_value) >>
        delta_neutral_settling_firm: cond!(version >=27 && deltaNeutra_order_type.unwrap_or("").is_empty(),string_value) >>
        delta_neutral_clearing_account: cond!(version >=27 && deltaNeutra_order_type.unwrap_or("").is_empty(), string_value) >>
        delta_neutral_clearing_intent: cond!(version >=27 && deltaNeutra_order_type.unwrap_or("").is_empty(), string_value) >>
        delta_neutral_open_close: cond!(version >=31 && deltaNeutra_order_type.unwrap_or("").is_empty(), string_value) >>
        delta_neutral_short_sale: cond!(version >=31 && deltaNeutra_order_type.unwrap_or("").is_empty(), bool_value) >>
        delta_neutral_short_sale_slot: cond!(version >=31 && deltaNeutra_order_type.unwrap_or("").is_empty(), int_value) >>
        delta_neutral_designated_location: cond!(version >=31 && deltaNeutra_order_type.unwrap_or("").is_empty(), string_value) >>
        continuous_update: cond!(version >= 11, int_value) >>
        stock_range_lower: cond!(version == 26, double_value) >>
        stock_range_upper: cond!(version == 26, double_value) >>
        reference_price_type: cond!(version >= 11, int_value) >>


        trail_stop_price: cond!(version >= 13, double_max_value) >>
        tailing_percent: cond!(version >= 30, double_max_value) >>

        basis_points: cond!(version >= 14, double_max_value) >>
        basis_points_type: cond!(version >= 14, double_max_value) >>
        combo_legs_descrip: cond!(version >= 14, string_value) >>

        // version >= 29
        combo_legs: cond!(version >= 29, decode_combo_leg_list) >>
        order_combo_legs: cond!(version >= 29, decode_order_combo_leg_list) >>

        // version >= 26
        smart_combo_routing_param_list: cond!(version >= 26, decode_tag_value_list) >>

        // version >= 15
        scale_init_level_size: cond!(version >= 15, int_max_value) >>
        scale_subs_level_size: cond!(version >= 15, int_max_value) >>
        scale_price_increment: cond!(version >= 15, double_max_value) >>

  
        // version >= 28
        scale_price_adjust_value: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                    double_max_value ) >>
        scale_price_adjust_interval: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                      int_max_value ) >>
        scale_profit_offset: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                    double_max_value ) >>
        scale_auto_reset: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                    bool_value ) >>
        scale_init_position: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                    int_max_value ) >>
        scale_init_fill_qty: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                    int_max_value ) >>
        scale_random_percent: cond!(version >= 28 && scale_price_increment.unwrap() > 0.0 && scale_price_increment.unwrap() != f64::MAX,
                                    bool_value ) >>

        // version >= 24
        hedge_type: cond!(version >= 24, string_value) >>
        hedge_param: cond!(version >= 24 && !hedge_type.unwrap().is_empty(), string_value) >>

        // version >= 25
        opt_out_smart_routing: cond!(version >= 25, bool_value) >>

        clearing_account: cond!(version >= 19, string_value) >>
        clearing_intent:  cond!(version >= 19, string_value) >>

        not_held: cond!(version >= 22, bool_value) >>

        under_comp: cond!(version >= 20, decode_under_comp) >>

        algo_strategy: cond!(version >= 21, string_value) >>
        algo_params_list: cond!(version >= 21 && !algo_strategy.unwrap().is_empty(), decode_tag_value_list) >>

        solicited: cond!(version >= 33, bool_value) >>

        what_if: cond!(version >= 16, bool_value) >>
        order_state: cond!(version >= 16, decode_order_state) >>

        randomize_size:  cond!(version >= 34, bool_value) >>
        randomize_price: cond!(version >= 34, bool_value) >>

        peg_info: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK && order_type == "PEG BENCH", decode_peg_bench_order) >>

        conditions: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, decode_conditions) >>

        adjusted_order_type: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, string_value) >>
        trigger_price: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, double_max_value) >>
        trail_stop_price: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, double_max_value) >>
        lmt_price_offset: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, double_max_value) >>
        adjusted_stop_price: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, double_max_value) >>
        adjusted_trailing_amount: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, double_max_value) >>
        adjustable_trailing_unit: cond!(server_version > MIN_SERVER_VER_PEGGED_TO_BENCHMARK, int_value) >>

        soft_dollar_tier_name: cond!(server_version > MIN_SERVER_VER_SOFT_DOLLAR_TIER, string_value) >>
        soft_dollar_tier_val: cond!(server_version > MIN_SERVER_VER_SOFT_DOLLAR_TIER, string_value) >>
        soft_dollar_tier_display_name: cond!(server_version > MIN_SERVER_VER_SOFT_DOLLAR_TIER, string_value) >>
        

        ()

    )
);

named!(decode_open_order_end_msg<&[u8],()>,
    do_parse!(
        version: int_value >>
        ()
    )
);
