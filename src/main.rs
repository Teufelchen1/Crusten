mod encoder;
use encoder::encode_dataitem;

mod cbor;
use cbor::DataItem;

mod decoder;
use decoder::CborContext;

use crate::encoder::encode_dataitem_vector;

fn encode_array() -> Vec<u8> {
    let arr: [[u16; 2]; 3] = [[0, 1], [2, 3], [4, 5]];
    println!("Unencoded Array: {:?}", arr);

    let mut cbor_enc: Vec<u8> = Vec::new();
    cbor_enc.append(&mut &mut encode_dataitem(DataItem::Array(arr.len() as u64)));

    for el in arr {
        cbor_enc.append(&mut encode_dataitem(DataItem::Array(arr[0].len() as u64)));

        for e in el {
            cbor_enc.append(&mut encode_dataitem(DataItem::UInt(e as u64)));
        }
    }
    return cbor_enc;
}

fn main() {
    let cbor_enc: Vec<u8> = encode_array();
    print!("CBOR Encoded Input Vector: ");
    for el in cbor_enc.clone() {
        print!("{:02x}", el);
    }
    println!();

    let test_data: Vec<DataItem> = vec![
        DataItem::Array(4),
        DataItem::UInt(u64::MAX),
        DataItem::NUint(-6493),
        DataItem::Float(1.1),
        DataItem::Float(3286.4),
    ];

    println!("Test Input Data: {:?}", test_data);

    let cbor_enc_test_data: Vec<u8> = encode_dataitem_vector(test_data);
    print!("CBOR Encoded Input Vector: ");
    for el in cbor_enc_test_data.clone() {
        print!("{:02x}", el);
    }
    println!();

    let mut data = CborContext::new(&cbor_enc_test_data);
    loop {
        match data.next() {
            Some(DataItem::Array(_)) => print!("["),
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
