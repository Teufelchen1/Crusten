#[derive(PartialEq)]
pub enum DataItem<'a> {
    UInt(u64),       // Major 0
    NUint(i128),      // Major 1
    Bytes(&'a [u8]), // Major 2
    Text(&'a [u8]),  // Major 3
    Array(u64),      // Major 4
    //Map<Key, Value>(length, &'a [(Key, Value)]), // Major 5
    Tag(u64),   // Major 6
    Float(f64), // Major 7
    Simple(u8), // Major 7
    Break(),    // Major 7
    Underflow,
    EndArray,
    End,
}

impl DataItem<'_> {
    pub fn to_string(&self) -> String {
        match *self {
            DataItem::UInt(y) => y.to_string(),
            DataItem::NUint(y) => y.to_string(),
            DataItem::Bytes(y) => format!("\'{:}\'", String::from_utf8_lossy(y).into_owned()),
            DataItem::Text(y) => format!("\"{:}\"", String::from_utf8_lossy(y).into_owned()),
            DataItem::Array(y) => format!("Array({:})", y),
            DataItem::Float(y) => y.to_string(),
            DataItem::Simple(y) => y.to_string(),
            DataItem::Break() => "Break".to_string(),
            _ => "lol".to_string(),
        }
    }
}
