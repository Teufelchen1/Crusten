//type length = u64;
//type rawbytes<'a> = &'a [u8];
//type tag = u64;

enum DataItem {
    UInt(u64), // Major 0
    NUint(u64), // Major 1
    //Bytes(length, rawbytes<'a>), // Major 2
    //Text(length, &'a str), // Major 3
    //Array(length, &'a [DataItem<'a>]), // Major 4
    //Map<Key, Value>(length, &'a [(Key, Value)]), // Major 5
    //Tag(tag, DataItem<'a>), // Major 6
    Float(f64), // Major 7
    Simple(u8), // Major 7
    Break(), // Major 7
}

fn encode(x: u64) -> DataItem {
    DataItem::UInt(x)
}

fn decode(x: DataItem) -> String {
    match x {
        DataItem::UInt(y) => y.to_string(),
        _ => todo!(),
    }
}


fn main() {
    println!("Hello IETF: {:?}", decode(encode(13)));
}
