use crate::DataItem;
use half::f16;

fn encode_array(x: u64) -> Vec<u8> {
    let mut buf: [u8; 9] = [0; 9];
    if x < u8::MAX as u64 {
        if x < 24 {
            return vec![0x80 + x as u8];
        }
        buf[0] = 0x98;
        buf[1] = x as u8;
        return buf[0..2].to_vec();
    }
    if x < u16::MAX as u64 {
        buf[0] = 0x99;
        buf[1] = (x >> 8) as u8;
        buf[2] = x as u8;
        return buf[0..3].to_vec();
    }
    if x < u32::MAX as u64 {
        buf[0] = 0x9a;
        buf[1] = (x >> 24) as u8;
        buf[2] = (x >> 16) as u8;
        buf[3] = (x >> 8) as u8;
        buf[4] = x as u8;
        return buf[0..5].to_vec();
    }
    buf[0] = 0x9b;
    buf[1] = (x >> 56) as u8;
    buf[2] = (x >> 48) as u8;
    buf[3] = (x >> 40) as u8;
    buf[4] = (x >> 32) as u8;
    buf[5] = (x >> 24) as u8;
    buf[6] = (x >> 16) as u8;
    buf[7] = (x >> 8) as u8;
    buf[8] = x as u8;
    return buf.to_vec();
}

fn encode_u64(x: u64) -> Vec<u8> {
    let mut buf: [u8; 9] = [0; 9];
    if x < u8::MAX as u64 {
        if x < 24 {
            return vec![x as u8];
        }
        buf[0] = 0x18;
        buf[1] = x as u8;
        return buf[0..2].to_vec();
    }
    if x < u16::MAX as u64 {
        buf[0] = 0x19;
        buf[1] = (x >> 8) as u8;
        buf[2] = x as u8;
        return buf[0..3].to_vec();
    }
    if x < u32::MAX as u64 {
        buf[0] = 0x1a;
        buf[1] = (x >> 24) as u8;
        buf[2] = (x >> 16) as u8;
        buf[3] = (x >> 8) as u8;
        buf[4] = x as u8;
        return buf[0..5].to_vec();
    }
    buf[0] = 0x1b;
    buf[1] = (x >> 56) as u8;
    buf[2] = (x >> 48) as u8;
    buf[3] = (x >> 40) as u8;
    buf[4] = (x >> 32) as u8;
    buf[5] = (x >> 24) as u8;
    buf[6] = (x >> 16) as u8;
    buf[7] = (x >> 8) as u8;
    buf[8] = x as u8;
    return buf.to_vec();
}

fn encode_nu64(x: i128) -> Vec<u8> {
    let mut buf: [u8; 9] = [0; 9];
    if x > i8::MIN as i128 {
        if x >= -24 {
            return vec![((x ^ 0xFF) + 0x20) as u8];
        }
        buf[0] = 0x38;
        buf[1] = (x ^ 0xFF) as u8;
        return buf[0..2].to_vec();
    }
    if x > i16::MIN as i128 {
        buf[0] = 0x39;
        buf[1] = ((x >> 8) ^ 0xFF) as u8;
        buf[2] = (x ^ 0xFF) as u8;
        return buf[0..3].to_vec();
    }
    if x > i32::MIN as i128 {
        buf[0] = 0x3a;
        buf[1] = ((x >> 24) ^ 0xFF) as u8;
        buf[2] = ((x >> 16) ^ 0xFF) as u8;
        buf[3] = ((x >> 8) ^ 0xFF) as u8;
        buf[4] = (x ^ 0xFF) as u8;
        return buf[0..5].to_vec();
    }
    buf[0] = 0x3b;
    buf[1] = ((x >> 56) ^ 0xFF) as u8;
    buf[2] = ((x >> 48) ^ 0xFF) as u8;
    buf[3] = ((x >> 40) ^ 0xFF) as u8;
    buf[4] = ((x >> 32) ^ 0xFF) as u8;
    buf[5] = ((x >> 24) ^ 0xFF) as u8;
    buf[6] = ((x >> 16) ^ 0xFF) as u8;
    buf[7] = ((x >> 8) ^ 0xFF) as u8;
    buf[8] = (x ^ 0xFF) as u8;
    return buf.to_vec();
}

fn encode_f64(x: f64) -> Vec<u8> {
    let mut buf: [u8; 9] = [0; 9];
    let tmp: f16 = f16::from_f64(x);
    if (tmp.to_f64()) == x {
        let tmp: f16 = f16::from_f64(x);
        let bytes = tmp.to_be_bytes();
        buf[0] = 0xf9;
        for i in 0..bytes.len() {
            buf[i + 1] = bytes[i];
        }
        return buf[0..3].to_vec();
    }

    let tmp: f32 = x as f32;
    if (tmp as f64) == x {
        let tmp: f32 = x as f32;
        let bytes = tmp.to_be_bytes();
        buf[0] = 0xfa;
        for i in 0..bytes.len() {
            buf[i + 1] = bytes[i];
        }
        return buf[0..5].to_vec();
    }

    let bytes = x.to_be_bytes();
    buf[0] = 0xfb;
    for i in 0..bytes.len() {
        buf[i + 1] = bytes[i];
    }
    return buf.to_vec();
}

fn encode_simple_val(x: u8) -> Vec<u8> {
    match x {
        20 => vec![0xf4], // false
        21 => vec![0xf5], // true
        22 => vec![0xf6], // null
        23 => vec![0xf7], // undefined
        31 => vec![0xFF], // break
        _ => vec![],
    }
}

pub fn encode_dataitem(x: DataItem) -> Vec<u8> {
    match x {
        DataItem::UInt(x) => encode_u64(x),
        DataItem::NUint(x) => encode_nu64(x),
        DataItem::Array(x) => encode_array(x),
        DataItem::Float(x) => encode_f64(x),
        DataItem::Simple(x) => encode_simple_val(x),
        _ => todo!(),
    }
}

pub fn encode_dataitem_vector(data: Vec<DataItem>) -> Vec<u8> {
    let mut tmp: Vec<u8> = Vec::new();

    for el in data {
        tmp.append(&mut encode_dataitem(el));
    }
    return tmp;
}

fn convert_vec_to_val(x: Vec<u8>) -> u128 {
    let mut res: u128 = 0;
    for el in x {
        res = res << 8 | (el as u128);
    }
    res
}

#[test]
fn test_encode_single_u8() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(0))) == 0x00);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(1))) == 0x01);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(10))) == 0x0a);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(23))) == 0x17);
}

#[test]
fn test_encode_two_byte_u8() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(24))) == 0x1818);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(25))) == 0x1819);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(100))) == 0x1864);
}

#[test]
fn test_encode_u16() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(1000))) == 0x1903e8);
}

#[test]
fn test_encode_u32() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(1000000))) == 0x1a000f4240);
}

#[test]
fn test_encode_u64() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(1000000000000))) == 0x1b000000e8d4a51000);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::UInt(u64::MAX))) == 0x1bffffffffffffffff);
}

#[test]
fn test_encode_single_nu8() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::NUint(-1))) == 0x20);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::NUint(-10))) == 0x29);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::NUint(-24))) == 0x37);
}

#[test]
fn test_encode_two_byte_nu8() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::NUint(-100))) == 0x3863);
}

#[test]
fn test_encode_nu16() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::NUint(-1000))) == 0x3903e7);
}

#[test]
fn test_encode_nu32() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::NUint(-1000000))) == 0x3a000f423f);
}

#[test]
fn test_encode_nu64() {
    assert!(
        convert_vec_to_val(encode_dataitem(DataItem::NUint(-18446744073709551616))) == 0x3bffffffffffffffff
    );
}

#[test]
fn test_encode_f16() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(0.0))) == 0xf90000);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(-0.0))) == 0xf98000);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(1.0))) == 0xf93c00);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(1.5))) == 0xf93e00);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(65504.0))) == 0xf97bff);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(-4.0))) == 0xf9c400);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(5.960464477539063e-8))) == 0xf90001);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(0.00006103515625))) == 0xf90400);
}

#[test]
fn test_encode_f32() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(100000.0))) == 0xfa47c35000);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(3.4028234663852886e+38))) == 0xfa7f7fffff);
}

#[test]
fn test_encode_f64() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(1.1))) == 0xfb3ff199999999999a);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(1.0e+300))) == 0xfb7e37e43c8800759c);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Float(-4.1))) == 0xfbc010666666666666);
}

#[test]
fn test_encode_array_size() {
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Array(0))) == 0x80);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Array(23))) == 0x97);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Array(24))) == 0x9818);
    assert!(convert_vec_to_val(encode_dataitem(DataItem::Array(1000))) == 0x9903e8);
}

#[test]
fn test_encode_array() {
    let arr: [[u16; 2]; 3] = [[0, 1], [2, 3], [4, 5]];
    let mut cbor_enc: Vec<u8> = Vec::new();
    cbor_enc.append(&mut encode_dataitem(DataItem::Array(arr.len() as u64)));

    for el in arr {
        cbor_enc.append(&mut encode_dataitem(DataItem::Array(arr[0].len() as u64)));

        for e in el {
            cbor_enc.append(&mut encode_dataitem(DataItem::UInt(e as u64)));
        }
    }
}
