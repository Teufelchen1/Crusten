//type length = u64;
//type rawbytes<'a> = &'a [u8];
//type tag = u64;

mod cbor;
use cbor::DataItem;

mod decoder;
use decoder::{decode_single_item, CborContext};

fn encode(x: u64) -> DataItem<'static> {
    DataItem::UInt(x)
}

fn main() {
    let mut data = CborContext::new(&[0x10]);
    loop {
        match data.next() {
            Some(DataItem::Array(y)) => print!("["),
            Some(DataItem::EndArray) => {
                print!("]");
                if data.has_next() {
                    print!(", ");
                };
            }
            Some(DataItem::Underflow) => panic!("Underflow!"),
            Some(DataItem::End) => break,
            Some(n) => {
                print!("{:}", n.to_string());
                if data.has_next() {
                    print!(", ");
                };
            }
            _ => todo!(),
        }
    }
    println!("");
}

#[test]
fn test_single_byte_uint() {
    assert!(matches!(decode_single_item(&[0x10]), DataItem::UInt(16)));
}

#[test]
fn test_one_argument_byte_uint() {
    assert!(matches!(
        decode_single_item(&[0x18, 24]),
        DataItem::UInt(24)
    ));
    assert!(matches!(
        decode_single_item(&[0x18, 200]),
        DataItem::UInt(200)
    ));
    assert!(matches!(
        decode_single_item(&[0x18, 255]),
        DataItem::UInt(255)
    ));
}

#[test]
fn test_two_argument_byte_uint() {
    assert!(matches!(
        decode_single_item(&[0x19, 1, 0]),
        DataItem::UInt(256)
    ));
    assert!(matches!(
        decode_single_item(&[0x19, 2, 2]),
        DataItem::UInt(514)
    ));
    assert!(matches!(
        decode_single_item(&[0x19, 255, 255]),
        DataItem::UInt(65535)
    ));
}

#[test]
fn test_four_argument_byte_uint() {
    assert!(matches!(
        decode_single_item(&[0x1A, 1, 0, 0, 0]),
        DataItem::UInt(16777216)
    ));
    assert!(matches!(
        decode_single_item(&[0x1A, 2, 2, 2, 2]),
        DataItem::UInt(33686018)
    ));
    assert!(matches!(
        decode_single_item(&[0x1A, 255, 255, 255, 255]),
        DataItem::UInt(4294967295)
    ));
}

#[test]
fn test_eight_argument_byte_uint() {
    assert!(matches!(
        decode_single_item(&[0x1B, 255, 255, 255, 255, 255, 255, 255, 255]),
        DataItem::UInt(18446744073709551615)
    ));
}

#[test]
fn test_float() {
    assert!(matches!(
        decode_single_item(&[0xF9, 0x00, 0x00]),
        DataItem::Float(0.0)
    ));
    assert!(matches!(
        decode_single_item(&[0xFB, 0x3F, 0xD5, 0x4F, 0xDF, 0x3B, 0x64, 0x5A, 0x1D]),
        DataItem::Float(0.333)
    ));
    assert!(matches!(
        decode_single_item(&[0xFB, 0x40, 0x58, 0xFF, 0x5C, 0x28, 0xF5, 0xC2, 0x8F]),
        DataItem::Float(99.99)
    ));
    assert!(matches!(
        decode_single_item(&[0xF9, 0x3E, 0x00]),
        DataItem::Float(1.5)
    ));
}
