use decoder::wire::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickSizeMsg {
    version: i32,
    tick_id: i32,
    tick_type: i32,
    size: i32,
}

named!(decode_tick_size<&[u8], TickSizeMsg>,
    do_parse!(
        version:    int_value >>
        tick_id:    int_value >>
        tick_type:  int_value >>
        size:       int_value >>
        (TickSizeMsg {
            version,
            tick_id,
            tick_type,
            size
        })
    )
);

#[test]
fn test_tick_size() {
    let input = &b"1\02\03\04\0"[..];
    let expect = TickSizeMsg {
        version: 1,
        tick_id: 2,
        tick_type: 3,
        size: 4,
    };

    assert_eq!(decode_tick_size(input), Ok((&b""[..], expect)));
}
