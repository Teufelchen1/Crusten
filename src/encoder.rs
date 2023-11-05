use half::f16;

pub enum DataItem {
    UInt(u64), // Major 0
    NUint(i128), // Major 1
    //Bytes(length, rawbytes<'a>), // Major 2
    //Text(length, &'a str), // Major 3
    Array(u64), // Major 4
    //Map<Key, Value>(length, &'a [(Key, Value)]), // Major 5
    //Tag(tag, DataItem<'a>), // Major 6
    Float(f64), // Major 7
    Simple(u8), // Major 7
}

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

fn encode_nu64(x:i128) -> Vec<u8> {
    let mut buf: [u8; 9] = [0; 9];
    if x > i8::MIN as i128 {
        if x >= -24 {
            return vec![((x ^ 0xFF) + 0x20) as u8]
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
            buf[i+1] = bytes[i];
        }
        return buf[0..3].to_vec()
    }

    let tmp: f32 = x as f32;
    if (tmp as f64) == x {
        let tmp: f32 = x as f32;
        let bytes = tmp.to_be_bytes();
        buf[0] = 0xfa;
        for i in 0..bytes.len() {
            buf[i+1] = bytes[i];
        }
        return buf[0..5].to_vec()
    }

    let bytes = x.to_be_bytes();
    buf[0] = 0xfb;
    for i in 0..bytes.len() {
        buf[i+1] = bytes[i];
    }
    return buf.to_vec();
}

fn encode_simple_val(x: u8) -> Vec<u8> {
    match x {
        20 => vec![0xf4],   // false
        21 => vec![0xf5],   // true
        22 => vec![0xf6],   // null
        23 => vec![0xf7],   // undefined
        31 => vec![0xFF],   // break
        _ => vec![],
    }
}

pub fn encode(x: DataItem) -> Vec<u8> {
    match x {
        DataItem::UInt(x) => encode_u64(x),
        DataItem::NUint(x) => encode_nu64(x),
        DataItem::Array(x) => encode_array(x),
        DataItem::Float(x) => encode_f64(x),
        DataItem::Simple(x) => encode_simple_val(x),
    }
}
