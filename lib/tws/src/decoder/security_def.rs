use decoder::wire::*;

named!(decode_security_definition_optional_parameter_msg<&[u8], ()>,
    do_parse!(
        req_id: int_value >>
        exchange: string_value >>
        underlying_con_id: int_value >>
        trading_class: string_value >>
        multiplier: string_value >>
        expirations: length_count!(int_value, string_value) >>
        strikes: length_count!(int_value, double_value) >>
        ()
    )
);

named!(decode_security_definition_optional_parameter_end_msg<&[u8], ()>,
    do_parse!(
        req_id: int_value >>

        ()
    )
);
