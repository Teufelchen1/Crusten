//type length = u64;
//type rawbytes<'a> = &'a [u8];
//type tag = u64;

mod cbor;
use cbor::DataItem;

mod decoder;
use decoder::CborContext;

fn encode(x: u64) -> DataItem<'static> {
    DataItem::UInt(x)
}

fn main() {
    let mut data = CborContext::new(&[0x10]);
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
