use decoder::wire::*;

named!(decode_verify_and_auth_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        api_data: string_value >>
        xyz_challenge: string_value >>
        ()
    )
);

named!(decode_verify_and_auth_complete_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        is_successful: map!(string_value, |s|  {if s == "true" {true} else {false} })>>
        error_text: string_value >>
        ()
    )
);

named!(decode_verify_api_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        api_data: string_value >>
        ()
    )
);

named!(decode_verify_api_complete_msg<&[u8], ()>,
    do_parse!(
        version: int_value >>
        is_successful: map!(string_value, |s|  {if s == "true" {true} else {false} })>>
        error_text: string_value >>
        ()
    )
);
