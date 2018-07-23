use decoder::wire::*;

named!(decode_order_state_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        id: int_value >>
        status: string_value >>
        filled: double_value >>
        remaining: double_value >>
        avg_fill_price: double_value >>

        perm_id: cond!(version >= 2, int_value) >>

        parent_id: cond!(version >= 3, int_value) >>

        last_fill_price: cond!(version >= 4, double_value) >>

        client_id: cond!(version >= 5, int_value) >>

        why_held: cond!(version >= 6, string_value) >>

        ()
    )
);
