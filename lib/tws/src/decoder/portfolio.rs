use decoder::wire::*;

named_args!(decode_portfolio<'a>(server_version: i32)<&'a [u8], ()>,
    do_parse!(
        version: int_value >>
        con_id: cond!(version >= 6, int_value) >>
        symbol: string_value >>
        sec_type: string_value >>
        last_trade_date_or_contract_month: string_value >>
        strike: double_value >>
        right: string_value >>
        multiplier: cond!(version >= 7, string_value) >>
        primary_exch: cond!(version >= 7, string_value) >>
        currency: string_value >>
        local_symbol: cond!(version >= 2, string_value) >>
        trading_class: cond!(version >= 8, string_value) >>
        position: double_value >>
        market_price: double_value >>
        market_value: double_value >>
        average_cost: cond!(version >= 3, double_value) >>
        unrealized_pnl: cond!(version >= 3, double_value) >>
        realized_pnl: cond!(version >= 3, double_value) >>

        account_name: cond!(version >= 4, string_value) >>

        primary_exch: cond!(version ==6 && server_version == 39,  string_value) >>
        ()
    )
);
