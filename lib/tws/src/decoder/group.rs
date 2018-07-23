use decoder::wire::*;

named!(decode_display_group_update_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        contract_info: string_value >>
        ()
    )
);

named!(decode_display_group_list_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        req_id: int_value >>
        groups: string_value >>
        ()
    )
);
