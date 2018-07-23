use decoder::wire::*;
use tick_type::*;

named!(decode_tick_option)computation_msg<&[u8], ()>,
    do_parse!(
        version : int_value >>
        ticker_id: int_value >>
        tick_type: int_value >>
        implied_vol: double_value >>
        delta: double_value >>
        opt_price: cond!(version >= 6 || tick_type == MODEL_OPTION, double_value) >>
        pv_dividend: cond!(version >= 6 || tick_type == MODEL_OPTION, double_value) >>
        
        gamma: cond!(version >= 6, double_value) >>
        vega: cond!(version >= 6, double_value) >>
        theta: cond!(version >= 6, double_value) >>
        und_price: cond!(version >= 6, double_value) >>

        ()
    )
);