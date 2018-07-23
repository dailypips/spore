use decoder::wire::*;
use penny::{Currency};
use commission_report::*;
use std::str::FromStr;

named!(decode_commission_report_msg<&[u8], CommissionReport>,
    do_parse!(
        version: int_value >>
        exec_id: string_value >>
        commission: double_value >>
        currency: string_value >>
        realized_pnl: double_value >>
        yield_value: double_value >>
        yield_redemption_date: int_value >>
        (CommissionReport {
            exec_id: exec_id.to_string(),
            commission,
            currency: Currency::from_str(currency).unwrap(),
            realized_pnl, yield_value,
            yield_redemption_date

        })
    )
);

named!(decode_market_data_type_msg<&[u8], MarketDataType>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        market_data_type: int_value >>
        (MarketDataType {
            req_id,
            market_data_type
        })
    )
);

named!(decode_tick_shapshot_end_msg<&[u8], i32>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        (req_id)
    )
);

named!(decode_delta_netrual_validation_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        con_id: int_value >>
        delta: double_value >>
        price: double_value >>
        ()
    )
);

named!(decode_delta_execution_data_end_msg<&[u8], i32>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        (req_id)
    )
);

named!(decode_acct_download_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        account_name: string_value >>
        ()
    )
);

named!(decode_contract_data_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        ()
    )
);

named!(decode_fundamental_data_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        data: string_value >>
        ()
    )
);

named!(decode_real_time_bar_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        time: long_value >>
        open: double_value >>
        high: double_value >>
        low: double_value >>
        close: double_value >>
        volume: long_value >>
        wap: double_value >>
        count: int_value >>
        ()
    )
);

named!(decode_current_time_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        time: long_value >>
        ()
    )
);

named!(decode_scanner_parameters_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        xnl: string_value >>
        ()
    )
);

named_args!(decode_historical_data_item<'a>(version: i32)<&'a [u8], ()>,
    do_parse!(
        date: string_value >>
        open: double_value >>
        high: double_value >>
        low: double_value >>
        close: double_value >>
        volume: int_value >>
        wap: double_value >>
        has_gaps: string_value >>
        bar_count: cond!(version >= 3, int_value) >>
        ()
    )
);

named!(decode_historical_data_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        start_date: cond!(version >= 2, string_value) >>
        end_date: cond!(version >= 2, string_value) >>
        data: length_count!(int_value, call!(decode_historical_data_item, version)) >>
        ()
    )
);

named!(decode_historical_data_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        ()
    )
);

named!(decode_receive_fa_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        fa_data_type: int_value >>
        xml: string_value >>
        ()
    )
);

named!(decode_managed_accts_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        accts_list: string_value >>
        ()
    )
);

named!(decode_news_bulletins_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        msg_id: int_value >>
        msg_type: int_value >>
        message: string_value >>
        originating_exch: string_value >>
        ()
    )
);

named!(decode_market_depth_l2_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        id: int_value >>
        position: int_value >>
        market_maker: string_value >>
        operation: int_value >>
        side: int_value >>
        price: double_value >>
        size: int_value >>
        ()
    )
);
named!(decode_market_depth_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        id: int_value >>
        position: int_value >>
        operation: int_value >>
        side: int_value >>
        price: double_value >>
        size: int_value >>
        ()
    )
);

named!(decode_next_valid_id_msg<&[u8], i32>,
    do_parse!(
        version: int_value >>
        order_id: int_value >>
        (order_id)
    )
);

named!(decode_err_msg<&[u8], ErrorMsg>,
    do_parse!(
        version: int_value >>
        v1_msg: cond!(version < 2, string_value) >>
        err_id: int_value >>
        err_code: int_value >>
        err_msg: string_value >>
        ({
            if version < 2 {
                ErrorMsg {
                    id: -1,
                    code: -1,
                    message: v1_msg.unwrap().to_string()
                }
            }else {
                ErrorMsg {
                    id: err_id,
                    code: err_code,
                    message: err_msg.to_string()
                }
            }
        })
    )
);

named!(decode_acct_update_time_msg<&[u8], String>,
    do_parse!(
        version: int_value >>
        time_stamp: string_value >>
        (time_stamp.to_string())
    )
);

named!(decode_acct_value_msg<&[u8], AccountValue>,
    do_parse!(
        version: int_value >>
        key: string_value >>
        val: string_value >>
        cur: string_value >>
        account_name: cond!(version >= 2, string_value) >>
        (AccountValue {
            account: account_name.map(|s| s.to_string()),
            key: key.to_string(),
            value: val.to_string(),
            cur: cur.to_string()
        })
    )
);

named!(decode_tick_efp_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        tick_id: int_value >>
        tick_type: int_value >>
        basis_points: double_value >>
        formatted_basis_points: string_value >>
        implied_futures_price: double_value >>
        hold_days: int_value >>
        future_last_trade_date: string_value >>
        dividen_impact: double_value >>
        dividens_to_last_trade_data: double_value >>
        ()
    )
);

named!(decode_tick_string_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        tick_id: int_value >>
        tick_type: int_value >>
        value: string_value >>
        ()
    )
);

named!(decode_tick_generic_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        tick_id: int_value >>
        tick_type: int_value >>
        value: double_value >>
        ()
    )
);

named!(decode_acct_summary_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        ()
    )
);

named!(decode_acct_summary_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        account: string_value >>
        tag: string_value >>
        value: string_value >>
        currency: string_value >>
        ()
    )
);

named!(decode_account_update_multi_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        account: string_value >>
        model_code: string_value >>
        key: string_value >>
        value: string_value >>
        currency: string_value >>
        ()
    )
);

named!(decode_account_update_multi_end_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        ()
    )
);
