use decoder::constants::*;
use decoder::wire::*;
use nom::IResult;

named!(pub decode_ack<&[u8], (i32, &str)>,
    do_parse!(
        version: int_value >>
        addr_or_time: string_value >>
        (version, addr_or_time)
    )
);

named_args!(pub decode_message(server_version: i32)<&[u8], ()>,
      map!( int_value, |msg_id| {
        match msg_id {
                   END_CONN => (),
                   TICK_PRICE => (),
                   TICK_SIZE => (),
                   ORDER_STATUS => (),
                   ERR_MSG => (),
                   OPEN_ORDER => (),
                   ACCT_VALUE => (),
                   PORTFOLIO_VALUE => (),
                    ACCT_UPDATE_TIME => (),
                    NEXT_VALID_ID => (),
                    CONTRACT_DATA => (),
                    EXECUTION_DATA => (),
                    MARKET_DEPTH => (),
                    MARKET_DEPTH_L2 => (),
                    NEWS_BULLETINS => (),
                    MANAGED_ACCTS => (),
                    RECEIVE_FA => (),
                    HISTORICAL_DATA => (),
                    BOND_CONTRACT_DATA => (),
                    SCANNER_PARAMETERS => (),
                    SCANNER_DATA => (),
                    TICK_OPTION_COMPUTATION => (),
                    TICK_GENERIC => (),
                    TICK_STRING => (),
                    TICK_EFP => (),
                    CURRENT_TIME => (),
                    REAL_TIME_BARS => (),
                    FUNDAMENTAL_DATA => (),
                    CONTRACT_DATA_END => (),
                    OPEN_ORDER_END => (),
                    ACCT_DOWNLOAD_END => (),
                    EXECUTION_DATA_END => (),
                    DELTA_NEUTRAL_VALIDATION => (),
                    TICK_SNAPSHOT_END => (),
                    MARKET_DATA_TYPE => (),
                    COMMISSION_REPORT=> (),
                    POSITION => (),
                    POSITION_END => (),
                    ACCOUNT_SUMMARY => (),
                    ACCOUNT_SUMMARY_END => (),
                    VERIFY_MESSAGE_API => (),
                    VERIFY_COMPLETED => (),
                    DISPLAY_GROUP_LIST => (),
                    DISPLAY_GROUP_UPDATED => (),
                    VERIFY_AND_AUTH_MESSAGE_API => (),
                    VERIFY_AND_AUTH_COMPLETED => (),
                    POSITION_MULTI => (),
                    POSITION_MULTI_END => (),
                    ACCOUNT_UPDATE_MULTI => (),
                    ACCOUNT_UPDATE_MULTI_END => (),
                    SECURITY_DEFINITION_OPTION_PARAMETER => (),
                    SECURITY_DEFINITION_OPTION_PARAMETER_END => (),
                    SOFT_DOLLAR_TIERS => (),
                   _ => (),
               }
           }
       )
);
