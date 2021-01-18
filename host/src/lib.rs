#[derive(Debug)]
pub enum ClrObject {
  Char(u8),
  Byte(u8),
  SByte(i8),
  Boolean(bool),
  Short(i16),
  UShort(u16),
  Int32(i32),
  UInt32(u32),
  Int64(i64),
  UInt64(u64),
  Float(f32),
  Double(f32),
  Decimal(f64),
  String { value: *const u8, length: u64 },
}

// impl From<String> for ClrObject {
//   fn from(value: String) -> Self {
//     let value_ptr = value.as_ptr();
//     let value_len = value.len() as _;

//     std::mem::forget(value);

//     Self::String {
//       value: value_ptr,
//       length: value_len,
//     }
//   }
// }
