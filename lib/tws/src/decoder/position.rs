use decoder::wire::*;
use tagvalue::*;

named_args!(pub decode_position_msg<'a>(server_version: i32)<&'a [u8], ()>,
    do_parse!(
        version: int_value >>
        account: string_value >>
        con_id: int_value >>
        symbol: string_value >>
        sec_type: string_value >>
        last_trade_date_or_contract_month: string_value >>
        strike: double_value >>
        right: string_value >>
        multiplier: string_value >>
        exchange: string_value >>
        currency: string_value >>
        local_symbol: string_value >>
        trading_class: cond!(version >= 2, string_value) >>
        pos: double_value >>
        avg_cost: cond!(version >= 3, double_value) >>
        ()
    )
);

named!(pub decode_position_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        ()
    )
);

named!(pub decode_position_multi_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        account: string_value >>

        con_id: int_value >>
        symbol: string_value >>
        sec_type: string_value >>
        last_trade_date_or_contract_month: string_value >>
        strike: double_value >>
        right: string_value >>
        multiplier: string_value >>
        exchange: string_value >>
        currency: string_value >>
        local_symbol: string_value >>
        trading_class: string_value >>
        pos: double_value >>
        avg_cost: double_value >>
        model_code: string_value >>
        ()

    )
);

named!(pub decode_position_multi_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        ()
        
    )
);