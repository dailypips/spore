use std::str::{from_utf8, FromStr};
use std::{f64, i32};

named!(string_literal, take_until_and_consume!("\0"));

named!(pub string_value<&[u8], &str>, map_res!(string_literal, from_utf8));

named!(pub int_value<&[u8], i32>,
    map_res!(map_res!(string_literal, from_utf8), |s| {
            i32::from_str_radix(s, 10)
    })
);

named!(pub int_max_value<&[u8], i32>,
    map_res!(map_res!(string_literal, from_utf8), |s : &str| {
        if s.is_empty() {
            Ok(i32::MAX)
        } else {
            i32::from_str_radix(s, 10)
        }
    })
);

named!(pub bool_value<&[u8], bool>,
    map!(int_value, |i| {
        if i != 0 {
            true
        }else {
            false
        }
    })
);

named!(pub long_value<&[u8], i64>,
    map_res!(map_res!(string_literal, from_utf8), |s| {
            i64::from_str_radix(s, 10)
    })
);

named!(pub double_value<&[u8], f64>,
    map_res!(map_res!(string_literal, from_utf8), |s| {
            f64::from_str(s)
    })
);

named!(pub double_max_value<&[u8], f64>,
    map_res!(map_res!(string_literal, from_utf8), |s :&str| {
        if s.is_empty() {
            Ok(f64::MAX)
        }else {
            f64::from_str(s)
        }
    })
);

#[test]
fn test_wired() {
    let null = &b""[..];
    assert_eq!(string_value(null).is_err(), true);
    assert_eq!(bool_value(null).is_err(), true);

    let empty = &b"\0"[..];
    assert_eq!(string_value(empty), Ok((&b""[..], "")));
    assert_eq!(int_max_value(empty), Ok((&b""[..], i32::MAX)));
    assert_eq!(double_max_value(empty), Ok((&b""[..], f64::MAX)));
    assert_eq!(long_value(empty).is_err(), true);
    assert_eq!(int_value(empty).is_err(), true);
    assert_eq!(double_value(empty).is_err(), true);
    assert_eq!(bool_value(empty).is_err(), true);

    let btrue = &b"1\0"[..];
    assert_eq!(bool_value(btrue), Ok((&b""[..], true)));

    let bfalse = &b"0\0"[..];
    assert_eq!(bool_value(bfalse), Ok((&b""[..], false)));

    let intvalue = &b"-123\0"[..];
    assert_eq!(string_value(intvalue), Ok((&b""[..], "-123")));
    assert_eq!(int_value(intvalue), Ok((&b""[..], -123)));
    assert_eq!(int_max_value(intvalue), Ok((&b""[..], -123)));
    assert_eq!(long_value(intvalue), Ok((&b""[..], -123)));
    assert_eq!(double_value(intvalue), Ok((&b""[..], -123.0)));
    assert_eq!(double_max_value(intvalue), Ok((&b""[..], -123.0)));

    let dvalue = &b"-123.456e7\0"[..];
    assert_eq!(double_value(dvalue), Ok((&b""[..], -123.456e7)));
    assert_eq!(double_max_value(dvalue), Ok((&b""[..], -123.456e7)));
    assert_eq!(long_value(dvalue).is_err(), true);
    assert_eq!(int_value(dvalue).is_err(), true);
    assert_eq!(int_max_value(dvalue).is_err(), true);
}
