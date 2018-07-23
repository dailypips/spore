use decoder::wire::*;
use tagvalue::*;

named!(decode_tag_value<&[u8], TagValue>,
    do_parse!(
        tag: string_value >> value: string_value >> (TagValue {
            tag: tag.to_string(),
            value: value.to_string()
        })
    )
);

named!(
    pub decode_tag_value_list < &[u8],
    Vec<TagValue> >,
    length_count!(int_value, decode_tag_value)
);

#[test]
fn test_tag_value_list() {
    let list = &b"3\0tag1\0value1\0tag2\0value2\0tag3\0value3\0"[..];
    let expect = vec![
        TagValue {
            tag: "tag1".to_string(),
            value: "value1".to_string(),
        },
        TagValue {
            tag: "tag2".to_string(),
            value: "value2".to_string(),
        },
        TagValue {
            tag: "tag3".to_string(),
            value: "value3".to_string(),
        },
    ];

    assert_eq!(decode_tag_value_list(list), Ok((&b""[..], expect)));
}
