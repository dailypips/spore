use contract::*;
use contract_detail::*;
use decoder::wire::*;
use order::*;
use order_state::*;
use std::{f64, i32};
use tagvalue::*;
use decoder::tagvalue::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickPriceMsg {
    version: i32,
    tick_id: i32,
    tick_type: i32,
    price: f64,
    size: i32,
    can_auto_execute: i32,
}

named!(decode_tick_price<&[u8], TickPriceMsg>,
    do_parse!(
        version:    int_value >>
        tick_id:    int_value >>
        tick_type:  int_value >>
        price:      double_value >>
        size:       cond!(version >= 2,  int_value) >>
        can_auto_execute: cond!(version >=3, int_value) >>
        (TickPriceMsg {
            version,
            tick_id, 
            tick_type, 
            price, 
            size: size.unwrap_or(0), 
            can_auto_execute: can_auto_execute.unwrap_or(0) 
    })
    )
);

#[test]
fn test_decode_tick_price() {
    let version1 = &b"1\02\03\01.2345\0"[..];
    let version2 = &b"2\02\03\01.2345\01234567\0"[..];
    let version3 = &b"3\02\03\01.2345\01234567\01\0"[..];

    let version1_expect = TickPriceMsg {
        version: 1,
        tick_id: 2,
        tick_type: 3,
        price: 1.2345,
        size: 0,
        can_auto_execute: 0,
    };

    let version2_expect = TickPriceMsg {
        version: 2,
        tick_id: 2,
        tick_type: 3,
        price: 1.2345,
        size: 1234567,
        can_auto_execute: 0,
    };

    let version3_expect = TickPriceMsg {
        version: 3,
        tick_id: 2,
        tick_type: 3,
        price: 1.2345,
        size: 1234567,
        can_auto_execute: 1,
    };

    assert_eq!(decode_tick_price(version1), Ok((&b""[..], version1_expect)));
    assert_eq!(decode_tick_price(version2), Ok((&b""[..], version2_expect)));
    assert_eq!(decode_tick_price(version3), Ok((&b""[..], version3_expect)));
}
