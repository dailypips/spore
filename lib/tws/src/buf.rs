use std::{f64, i32};
use tagvalue::TagValue;

pub struct EncodeBuf {
    pub(crate) buf: Vec<u8>,
}

impl EncodeBuf {
    pub fn new() -> Self {
        EncodeBuf { buf: Vec::new() }
    }

    pub fn put_string(&mut self, s: &String) {
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(0);
    }

    pub fn put_int(&mut self, i: i32) {
        self.put_string(&i.to_string());
    }

    pub fn put_int_max(&mut self, i: i32) {
        if i == i32::MAX {
            self.buf.push(0);
        } else {
            self.put_int(i);
        }
    }

    pub fn put_f64(&mut self, d: f64) {
        self.put_string(&d.to_string());
    }

    pub fn put_f64_max(&mut self, d: f64) {
        if d == f64::MAX {
            self.buf.push(0);
        } else {
            self.put_f64(d);
        }
    }

    pub fn put_bool(&mut self, b: bool) {
        if b {
            self.put_int(1);
        } else {
            self.put_int(0);
        }
    }

    pub fn put_tagvalue(&mut self, options: &Vec<TagValue>) {
        let count = options.len() as i32;
        self.put_int(count);
        if count > 0 {
            for elem in options {
                let s = format!("{}={};", elem.tag, elem.value);
                self.put_string(&s);
            }
            self.buf.push(0);
        }
    }
}
