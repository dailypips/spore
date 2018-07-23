use decoder::constants::*;
use decoder::wire::*;

named_args!(decode_execution_data_msg<'a>(server_version: i32)<&'a [u8],()>,
    do_parse!(
        version: int_value >>
        req_id: cond!(version >= 7, int_value) >>
        order_id: int_value >>
        
        // read contract
        con_id: cond!(version >= 5, int_value) >>
        symbol: string_value >>
        sec_type: string_value >>
        last_trade_date_or_contract_month: string_value >>
        strike: double_value >>
        right: string_value >>
        multiplier: cond!(version >= 9, string_value) >>
        exchange: string_value >>
        currency: string_value >>
        local_symbol: string_value >>
        trading_class: string_value >>

        // execution
        exec_id: string_value >>
        time: string_value >>
        acct_number: string_value >>
        exchange: string_value >>
        side: string_value >>
        shares: double_value >>
        price: double_value >>
        perm_id: cond!(version >= 2, int_value) >>
        client_id: cond!(version >= 3, int_value) >>
        liquidation: cond!(version >= 4, int_value) >>
        cum_qty: cond!(version >= 6, int_value) >>
        avg_price: cond!(version >= 6, double_value) >>
        order_ref: cond!(version >= 8, string_value) >>
        ev_rule: cond!(version >= 9, string_value) >>
        ev_multiplier: cond!(version >= 9, double_value) >>

        model_code: cond!(server_version >= MIN_SERVER_VER_MODELS_SUPPORT, string_value) >>
        ()

    )
);
