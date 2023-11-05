use crate::DataItem;

fn bytes_to_u64(t: &[u8]) -> u64 {
    let mut result: u64 = 0;
    for i in t {
        result = result << 8;
        result += *i as u64;
    }
    result
}

// https://stackoverflow.com/questions/36008434/how-can-i-decode-f16-to-f32-using-only-the-stable-standard-library
fn f16_to_f64(half: u16) -> f64 {
    let res = {
        let exp: u16 = half >> 10 & 0x1f;
        let mant: u16 = half & 0x3ff;
        let val: f32 = if exp == 0 {
            mant as f32 * (2.0f32).powi(-24)
        } else if exp != 31 {
            (mant as f32 + 1024f32) * (2.0f32).powi(exp as i32 - 25)
        } else if mant == 0 {
            ::std::f32::INFINITY
        } else {
            ::std::f32::NAN
        };
        if half & 0x8000 != 0 {
            -val
        } else {
            val
        }
    };
    f64::from(res)
}

pub fn decode(raw: &[u8]) -> (DataItem, usize) {
    let mut bytes_consumed = 1;
    let first_byte = raw[0];
    let major = (first_byte & 0b111_00000) >> 5;
    let additional_info = first_byte & 0b000_11111;

    let argument: u64 = match additional_info {
        0..=23 => additional_info.into(),
        24 => {
            bytes_consumed += 1;
            raw[1].into()
        },
        25 => {
            bytes_consumed += 2;
            bytes_to_u64(&raw[1..=2])
        },
        26 => {
            bytes_consumed += 4;
            bytes_to_u64(&raw[1..=4])
        },
        27 => {
            bytes_consumed += 8;
            bytes_to_u64(&raw[1..=8])
        },
        28 => panic!("Malformed CBOR reserved values"),
        29 => panic!("Malformed CBOR reserved values"),
        30 => panic!("Malformed CBOR reserved values"),
        31 => todo!(),
        _ => unreachable!(),
    };

    match (major, additional_info) {
        (0, _) => (DataItem::UInt(argument), bytes_consumed),
        (1, _) => (DataItem::NUint(argument + 1), bytes_consumed),
        (2, 0..=23) => {
            let bytes = &raw[bytes_consumed..=additional_info.into()];
            bytes_consumed += additional_info as usize;
            (DataItem::Bytes(bytes), bytes_consumed)
        },
        (2, _) => todo!(),
        (3, 0..=23) => {
            let bytes = &raw[bytes_consumed..=additional_info.into()];
            bytes_consumed += additional_info as usize;
            (DataItem::Text(bytes), bytes_consumed)
        }
        (3, _) => todo!(),
        (4, _) => (DataItem::Array(argument), bytes_consumed),
        (5, _) => todo!(),
        (6, _) => {

            (DataItem::Tag(argument), bytes_consumed)
        },
        (7, 0..=23) => (DataItem::Simple(argument as u8), bytes_consumed),
        (7, 25) => (DataItem::Float(f16_to_f64(argument as u16)), bytes_consumed),
        (7, 26) => (DataItem::Float(f64::from(argument as u32)), bytes_consumed),
        (7, 27) => (DataItem::Float(f64::from_bits(argument)), bytes_consumed),
        (7, _) => todo!(),
        (_, _) => unreachable!(),
    }
}

pub struct CborContext<'a> {
    raw: &'a [u8],
    index: usize,
    state: [u64; 4],
    depth: usize,
}

impl CborContext<'_> {
    pub fn new(data: &[u8]) -> CborContext {
        CborContext {
            raw: data,
            index: 0,
            state: [1, 0, 0, 0],
            depth: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        self.state[self.depth] > 0 
    }

    pub fn next(&mut self) -> Option<DataItem> {
        if self.state[self.depth] > 0 {
            self.state[self.depth] -= 1;
            if self.index >= self.raw.len() {
                return Some(DataItem::Underflow);
            }
            let (item, consumed) = decode(&self.raw[self.index..]);
            self.index += consumed;
            match item {
                DataItem::Array(y) => {
                    self.depth += 1;
                    if self.depth >= self.state.len() {
                        panic!("Maximum depth exceeded");
                    }
                    self.state[self.depth] = y;
                }
                _ => {}
            }
            Some(item)
        } else {
            if self.depth > 0 {
                self.depth -= 1;
                Some(DataItem::EndArray)
            } else {
                Some(DataItem::End)
            }
        }
    }
}

/*
fn decode_array(raw: &[u8]) {
    let mut index = 0;
    let mut depth = 0;
    let mut state = [1, 0, 0, 0];
    loop {
        if state[depth] > 0 {
            state[depth] -= 1;
            if index >= raw.len() {
                println!("Unexpected end of data");
                break;
            }
            let (item, consumed) = decode(&raw[index..]);
            index += consumed;
            match item {
                DataItem::Array(y) => {
                    depth += 1;
                    if depth >= state.len() {
                        panic!("Maximum depth exceeded");
                    }
                    state[depth] = y;
                    print!("[");
                }
                _ => {
                    print!("{:}", item.to_string());
                    if state[depth] > 0 {
                        print!(", ")
                    }
                }
            }
        } else {
            if depth > 0 {
                print!("]");
                depth -= 1;
            } else {
                break;
            }
            if state[depth] > 0 {
                print!(", ")
            }
        }
    }
    println!("");
}
*/

pub fn decode_single_item(raw: &[u8]) -> DataItem {
    decode(raw).0
}
