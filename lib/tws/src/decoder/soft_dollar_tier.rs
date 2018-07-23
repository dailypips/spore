use decoder::wire::*;
use order::*;


named!(decode_soft_dollar_tier<&[u8], SoftDollarTier>,
    do_parse!(
        name: string_value >>
        value: string_value >>
        display: string_value >>
        (SoftDollarTier{
            name: name.to_string(),
            value: value.to_string(),
            display_name: display.to_string()
        })
    )
);

named!(decode_soft_dollar_tier_list < &[u8], Vec<SoftDollarTier>>,
    length_count!(int_value, decode_soft_dollar_tier)
);

struct SoftDollarTierMsg {
    req_id: i32,
    soft_dollar: Vec<SoftDollarTier>,
}

named!(
    decode_soft_dollar_tier_msg < &[u8], SoftDollarTierMsg>,
    do_parse!(
            req_id: int_value >> 
            soft_dollar: decode_soft_dollar_tier_list >> 
            (SoftDollarTierMsg{
                req_id,
                soft_dollar
            })
        )
);
